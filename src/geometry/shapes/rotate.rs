use parry3d_f64::bounding_volume::AABB;

use crate::{
    geometry::{edges::Edges, faces::Faces, Shape},
    math::Point,
};

impl Shape for fj::Rotate {
    fn bounding_volume(&self) -> AABB {
        // TASK: Implement.
        todo!()
    }

    fn faces(&self, _tolerance: f64) -> Faces {
        // TASK: Implement.
        todo!()
    }

    fn edges(&self) -> Edges {
        // TASK: Implement.
        todo!()
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}