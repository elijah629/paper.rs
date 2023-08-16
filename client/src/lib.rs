pub mod components;
pub mod team_color;
pub mod utils;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{PresentMode, WindowMode},
};
use components::{
    character_controller::CharacterController,
    game::Game,
    line::Line,
    name::Name,
    player::{add_player_systems, Player},
    team::Team,
};
use team_color::TeamColor;
use utils::{mesh_to_vert::MeshToVert, set_panic_hook::set_panic_hook, unwrap_abort::UnwrapAbort};
use wasm_bindgen::prelude::*;

pub const BOARD_RADIUS: f32 = 200.;
pub const PLAYER_VELOCITY: f32 = 200.;
pub const PLAYER_SIZE: f32 = 18.;
pub const LINE_WIDTH: f32 = PLAYER_SIZE;

// smaller binary
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[derive(Debug)]
// pub struct GGRSConfig;
// impl Config for GGRSConfig {
//     type Input = Vec2;
//     type State = u8;
//     type Address = PeerId;
// }

#[wasm_bindgen(start)]
fn main() {
    set_panic_hook();
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                present_mode: PresentMode::AutoNoVsync,
                fit_canvas_to_parent: true,
                canvas: Some("#canvas".to_string()),
                ..default()
            }),
            ..default()
        }),
        FrameTimeDiagnosticsPlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, fps_counter);
    add_player_systems(&mut app);

    // GgrsPlugin::<GGRSConfig>::new()
    //     .with_input_system(player_input)
    //     .build(&mut app);

    app.init_resource::<Game>().run();
}

#[derive(Component)]
struct FpsText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        FpsText,
    ));

    game.team_mats.insert(
        TeamColor::Green,
        materials.add(ColorMaterial::from(Color::GREEN)),
    );

    let bounds_verts = Into::<Mesh>::into(shape::RegularPolygon::new(100., 17))
        .vertices_2d()
        .unwrap_abort();

    let bounds: Mesh2dHandle = meshes
        .add({
            let mut bounds = Mesh::new(PrimitiveTopology::TriangleList);
            bounds.set_vertices_2d(bounds_verts.clone());

            let mut indices = vec![];
            for i in 1..(bounds_verts.len() as u32 - 1) {
                indices.extend_from_slice(&[0, i + 1, i]);
            }

            bounds.set_indices(Some(Indices::U32(indices)));

            bounds
        })
        .into();

    let bounds_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: bounds.clone(),
            material: game.team_mats.get(&TeamColor::Green).unwrap_abort().clone(),
            ..default()
        })
        .id();

    let line_mesh: Mesh2dHandle = meshes
        .add({
            let mut line_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            line_mesh.set_vertices_raw(vec![[0., 0., 0.]]);

            line_mesh
        })
        .into();

    let line_entity = commands
        .spawn(MaterialMesh2dBundle {
            mesh: line_mesh.clone(),
            material: game.team_mats.get(&TeamColor::Green).unwrap_abort().clone(),
            ..default()
        })
        .id();

    commands.spawn(Camera2dBundle::default());
    let player = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)).into())
                    .into(),
                material: game.team_mats.get(&TeamColor::Green).unwrap_abort().clone(),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Team {
                color: TeamColor::Green,
                entity: Some(bounds_entity),
                points: bounds_verts,
            },
            Line {
                entity: Some(line_entity),
                ..default()
            },
            CharacterController::default(),
            Player {
                in_bounds: true,
                last_in_bounds: true,
                dead: false,
            },
        ))
        .id();

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Player",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
        Name(Some(player)),
    ));
}

fn latest_fm_pos(
    mut mouse_evr: EventReader<CursorMoved>,
    mut touch_evr: EventReader<TouchInput>,
) -> Option<Vec2> {
    let mut latest_pos = None;

    for e in mouse_evr.iter() {
        latest_pos = Some(e.position);
    }

    for e in touch_evr.iter() {
        latest_pos = Some(e.position);
    }

    latest_pos
}

fn fps_counter(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
