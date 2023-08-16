use crate::{
    latest_fm_pos,
    utils::{
        geo::{
            extend_poly_line::extend_poly_line,
            has_self_intersection::has_self_intersection,
            inside_polygon::InsidePolygon,
            triangulate::{triangulate_indices, triangulate_quad},
        },
        mesh_to_vert::MeshToVert,
        rotate_vec::RotateVec,
        unwrap_abort::UnwrapAbort,
    },
    LINE_WIDTH, PLAYER_SIZE, PLAYER_VELOCITY,
};
use bevy::{
    prelude::*,
    render::{mesh::Indices, primitives::Aabb},
    sprite::Mesh2dHandle,
    time::common_conditions::on_timer,
};
use itertools::Itertools;
use std::{f32::consts::FRAC_PI_2, time::Duration};

use super::{character_controller::CharacterController, line::Line, name::Name, team::Team};

#[derive(Component)]
pub struct Player {
    pub in_bounds: bool,
    pub last_in_bounds: bool,
    pub dead: bool,
}

impl Player {
    pub fn kill(&mut self) {
        info!("Player died");
        self.dead = true;
        let window = web_sys::window().unwrap_abort();
        window
            .alert_with_message("You died, refresh to test demo again")
            .unwrap_abort();
    }
}

pub fn add_player_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_input,
            player_nametag,
            (
                player_line_transitions,
                player_lines.run_if(on_timer(Duration::from_secs_f32(0.2))),
            )
                .chain(),
            player_out_of_bounds,
            player_deaths,
            // Misc
            camera_follow_player,
        ),
    );
}

pub fn player_input(
    mut transform: Query<(&Player, &mut Transform)>,
    mut controller: Query<&mut CharacterController>,
    timer: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_evr: EventReader<CursorMoved>,
    touch_evr: EventReader<TouchInput>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let player = transform.single_mut();
    if player.0.dead {
        return;
    }

    let mut player = player.1;
    let mut controller = controller.single_mut();
    let (camera, camera_transform) = camera.single();

    let direction = {
        let up = keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W);
        let left = keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A);
        let down = keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S);
        let right = keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D);

        let mut direction = if !up && !left && !down && !right {
            controller.direction
        } else {
            Vec2::ZERO
        };

        if let Some(target) = latest_fm_pos(mouse_evr, touch_evr) {
            let target = camera
                .viewport_to_world_2d(camera_transform, target)
                .unwrap_abort();
            direction = target - player.translation.truncate();
        }

        if up {
            direction += Vec2::Y;
        }

        if left {
            direction += Vec2::NEG_X;
        }

        if down {
            direction += Vec2::NEG_Y;
        }

        if right {
            direction += Vec2::X;
        }

        direction
    };

    controller.target_direction = direction.normalize_or_zero();
    controller.direction = controller.direction.lerp(controller.target_direction, 0.15);

    player.translation +=
        (controller.direction * PLAYER_VELOCITY * timer.delta_seconds()).extend(0.);

    let angle = controller.direction.y.atan2(controller.direction.x);
    player.rotation = Quat::from_axis_angle(Vec3::Z, angle).normalize();
}

pub fn player_nametag(
    mut nametag_query: Query<(&mut Transform, &Name)>,
    player_query: Query<&Transform, Without<Name>>,
) {
    for (mut text, name) in &mut nametag_query {
        if let Some(entity) = name.0 {
            let player = player_query.get(entity).unwrap_abort();
            text.translation = player.translation + (Vec3::Y * (PLAYER_SIZE + 5.));
        }
    }
}

pub fn player_lines(
    mut meshes: ResMut<Assets<Mesh>>,
    mut players: Query<(&Transform, &Player, &mut Line)>,
    mut lines: Query<(&Mesh2dHandle, &mut Aabb)>,
) {
    for (player_pos, player, mut line) in &mut players {
        if player.dead {
            continue;
        }

        if !player.in_bounds {
            let (line_mesh, mut aabb) = lines.get_mut(line.entity.unwrap_abort()).unwrap_abort();
            let mesh = meshes.get_mut(&line_mesh.0).unwrap_abort();

            line.points.push(player_pos.translation.truncate());

            let vertices: Vec<[f32; 3]> = {
                const HALF_WIDTH: f32 = LINE_WIDTH / 2.;
                let half_vec = Vec2::new(HALF_WIDTH, 0.);

                line.points
                    .iter()
                    .tuple_windows()
                    .map(|(&a, &b)| {
                        let diff = a - b;
                        let angle = diff.y.atan2(diff.x);
                        let p = a - half_vec;

                        let l = p.rotate_around(angle + FRAC_PI_2, a).extend(0.);
                        let r = p.rotate_around(angle - FRAC_PI_2, a).extend(0.);

                        (l.to_array(), r.to_array())
                    })
                    .tuple_windows()
                    .map(|(a, b)| triangulate_quad(&[a.0, a.1, b.0, b.1]))
                    .flatten()
                    .collect()
            };

            mesh.set_vertices_raw(vertices);

            if let Some(c) = mesh.compute_aabb() {
                *aabb = c;
            }
        }
    }
}

pub fn player_line_transitions(
    mut meshes: ResMut<Assets<Mesh>>,
    mut players: Query<(&Transform, &Player, &mut Team, &mut Line)>,
    mut lines_bounds: Query<(&Mesh2dHandle, &mut Aabb)>,
) {
    for (player_pos, player, mut team, mut line) in &mut players {
        if player.dead {
            continue;
        }
        if let Ok([(line_mesh, mut line_aabb), (bounds_mesh, mut bounds_aabb)]) =
            lines_bounds.get_many_mut([line.entity.unwrap_abort(), team.entity.unwrap_abort()])
        {
            // Enter
            if player.in_bounds && !player.last_in_bounds {
                line.points.push(player_pos.translation.truncate());
                let merged = extend_poly_line(&team.points, &line.points).unwrap_abort();

                let indices = triangulate_indices(&merged);

                team.points = merged.clone();

                let bounds_mesh = meshes.get_mut(&bounds_mesh.0).unwrap_abort();
                bounds_mesh.set_vertices_2d(merged);
                bounds_mesh.set_indices(Some(Indices::U32(indices)));

                *bounds_aabb = bounds_mesh.compute_aabb().unwrap_abort();

                // Reset line to none
                let line_mesh = meshes.get_mut(&line_mesh.0).unwrap_abort();
                line_mesh.set_vertices_raw(vec![[0., 0., 0.]]);
                *line_aabb = line_mesh.compute_aabb().unwrap_abort();

                line.points = vec![];
            }

            // Leave
            if !player.in_bounds && player.last_in_bounds {
                line.points.push(player_pos.translation.truncate());
            }
        }
    }
}

pub fn player_out_of_bounds(mut player_line: Query<(&Transform, &Team, &mut Player)>) {
    for (player_pos, team, mut player) in &mut player_line {
        if player.dead {
            continue;
        }
        player.last_in_bounds = player.in_bounds;
        player.in_bounds = player_pos.translation.truncate().is_inside_of(&team.points);
    }
}

pub fn player_deaths(mut lines: Query<(&Line, &mut Player)>) {
    for (line, mut player) in &mut lines {
        if player.dead {
            continue;
        }
        if !player.in_bounds {
            if has_self_intersection(&line.points) {
                player.kill();
            }
        }
    }
}

pub fn camera_follow_player(
    mut set: ParamSet<(
        Query<&mut Transform, With<Camera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let binding = set.p1();
    let player = binding.get_single().unwrap_abort().translation;
    let mut binding = set.p0();
    let mut camera = binding.get_single_mut().unwrap_abort();

    camera.translation = player.truncate().extend(1.);
}
