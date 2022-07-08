use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    objects::{Face, Surface},
    validation::{validate, Validated, ValidationConfig, ValidationError},
};
use fj_math::{Aabb, Point};

use super::ToShape;

impl ToShape for fj::Sketch {
    fn to_shape(
        &self,
        config: &ValidationConfig,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Validated<Vec<Face>>, ValidationError> {
        let surface = Surface::xy_plane();

        let sketch = match self.chain() {
            fj::Chain::PolyChain(poly_chain) => {
                let points =
                    poly_chain.to_points().into_iter().map(Point::from);

                Face::builder(surface)
                    .with_exterior_polygon(points)
                    .with_color(self.color())
                    .build()
            }
        };

        validate(vec![sketch], config)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        match self.chain() {
            fj::Chain::PolyChain(poly_chain) => Aabb::<3>::from_points(
                poly_chain
                    .to_points()
                    .into_iter()
                    .map(Point::from)
                    .map(Point::to_xyz),
            ),
        }
    }
}
