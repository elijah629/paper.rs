use bevy::prelude::*;

pub trait MeshToVert {
    fn vertices_2d(&self) -> Option<Vec<Vec2>>;
    fn set_vertices_2d(&mut self, x: Vec<Vec2>);

    fn vertices_raw(&self) -> Option<&[[f32; 3]]>;
    fn set_vertices_raw(&mut self, x: Vec<[f32; 3]>);
}

impl MeshToVert for Mesh {
    fn vertices_2d(&self) -> Option<Vec<Vec2>> {
        let points = self.vertices_raw()?;
        let mut v = points
            .into_iter()
            .map(|x| Vec2::new(x[0], x[1]))
            .collect::<Vec<_>>();

        v.push(v[0]);
        Some(v)
    }
    fn vertices_raw(&self) -> Option<&[[f32; 3]]> {
        let points = self.attribute(Mesh::ATTRIBUTE_POSITION)?;
        let points = points.as_float3()?;
        Some(points)
    }
    fn set_vertices_raw(&mut self, x: Vec<[f32; 3]>) {
        self.insert_attribute(Mesh::ATTRIBUTE_POSITION, x);
    }

    fn set_vertices_2d(&mut self, x: Vec<Vec2>) {
        let raw = x.into_iter().map(|x| [x.x, x.y, 0.]).collect::<Vec<_>>();
        self.set_vertices_raw(raw);
    }
}
