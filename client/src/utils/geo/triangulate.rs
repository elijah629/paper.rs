use bevy::prelude::*;
use itertools::Itertools;

use crate::utils::unwrap_abort::UnwrapAbort;

pub fn triangulate_quad(vertices: &[[f32; 3]; 4]) -> Vec<[f32; 3]> {
    [
        // Triangle 1
        vertices[0],
        vertices[1],
        vertices[2],
        // Triangle 2
        vertices[1],
        vertices[2],
        vertices[3],
    ]
    .to_vec()
}

pub fn triangulate_indices(m_points: &[Vec2]) -> Vec<u32> {
    // earcutr::flatten(data)
    let vertices = m_points
        .into_iter()
        .map(|x| x.to_array())
        .flatten()
        .collect_vec();

    earcutr::earcut(&vertices, &[], 2)
        .unwrap_abort()
        .into_iter()
        .map(|x| x as u32)
        .collect_vec()
}
