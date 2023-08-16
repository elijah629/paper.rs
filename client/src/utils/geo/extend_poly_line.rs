use super::{close_polygon::close_polygon, find_closest_point::find_closest_point};
use bevy::prelude::*;
use itertools::Itertools;

pub fn extend_poly_line(poly: &[Vec2], line: &[Vec2]) -> Option<Vec<Vec2>> {
    let line_vertices = line.len();
    if line_vertices > 0 {
        let start_point = find_closest_point(line[0], poly);
        let end_point = find_closest_point(line[line_vertices - 1], poly);

        // Clockwize area

        // Select redundant vertices
        let mut redundant_vertices = vec![];

        let mut i = start_point;
        while i != end_point {
            if i == poly.len() {
                if end_point == 0 {
                    break;
                }

                i = 0;
            }
            redundant_vertices.push(poly[i]);

            i += 1;
        }

        // Close polygon
        redundant_vertices.push(poly[end_point]);

        // Add new vertices to clockwise temp area
        let mut cw_points = poly.to_vec();

        for i in 0..line_vertices {
            cw_points.insert(i + start_point, line[i]);
        }

        // Remove the redundat vertices & calculate clockwise area's size
        cw_points = cw_points
            .iter()
            .filter(|x| !redundant_vertices.contains(x))
            .map(|&x| x)
            .collect_vec();

        let cw_area = (cw_points
            .windows(2)
            .map(|p| (p[1].x - p[0].x) * (p[1].y + p[0].y))
            .sum::<f32>()
            / 2.)
            .abs();

        // Counterclockwise area

        // Select redundant vertices
        redundant_vertices.clear();

        let mut i = start_point as isize;

        while i != end_point as isize {
            if i == -1 {
                if end_point == poly.len() - 1 {
                    break;
                }

                i = (poly.len() - 1) as isize;
            }
            redundant_vertices.push(poly[i as usize]);

            i -= 1;
        }
        redundant_vertices.push(poly[end_point]);

        // Add new vertices to clockwise temp area
        let mut ccw_points = poly.to_vec();
        for i in 0..line_vertices {
            ccw_points.insert(start_point, line[i]);
        }

        // Remove the redundant vertices & calculate counterclockwise area's size
        ccw_points = ccw_points
            .iter()
            .filter(|x| !redundant_vertices.contains(x))
            .map(|&x| x)
            .collect_vec();

        let ccw_area = (ccw_points
            .windows(2)
            .map(|p| (p[1].x - p[0].x) * (p[1].y + p[0].y))
            .sum::<f32>()
            / 2.)
            .abs();

        // Find the area with greatest size
        return Some(if cw_area > ccw_area {
            close_polygon(&cw_points)
        } else {
            close_polygon(&ccw_points)
        });
    }
    None
}
