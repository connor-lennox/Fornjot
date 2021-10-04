use std::cmp::min;

use decorum::R32;

use crate::math::Point;

/// A polygon without holes
///
/// `D` defines the dimension of the polygon's points. `N` defines the number of
/// points of the polygon.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Polygon<const D: usize, const N: usize> {
    points: [nalgebra::Point<R32, D>; N],
}

// TASK: Make generic over `N`.
impl<const D: usize> Polygon<D, 3> {
    /// Create a new `Triangle`
    pub fn new(points: [impl Into<Point<D>>; 3]) -> Result<Self, Error> {
        let points = points.map(|point| point.into());

        for (i, a) in points.iter().enumerate() {
            for b in points[i + 1..].iter() {
                if a == b {
                    return Err(Error::IdenticalPoints);
                }
            }
        }

        // This can be made a bit nicer using `array_windows` once that becomes
        // stable.
        for abc in points.windows(3) {
            let mut abc = abc.iter();

            let a = abc.next().unwrap();
            let b = abc.next().unwrap();
            let c = abc.next().unwrap();

            if (b - a).normalize() == (c - b).normalize() {
                return Err(Error::PointsOnLine);
            }
        }

        let [a, b, c] = points.map(|point| point.map(|coord| coord.into()));

        let min = min(a.coords.data.0, min(b.coords.data.0, c.coords.data.0));
        let min = nalgebra::Point::from(min[0]);

        let (a, b, c) = if a == min {
            (a, b, c)
        } else if b == min {
            (b, c, a)
        } else {
            (c, a, b)
        };

        Ok(Self { points: [a, b, c] })
    }

    /// Return the points of the triangle
    pub fn points(&self) -> [Point<D>; 3] {
        self.points.map(|point| point.map(|coord| coord.into()))
    }
}

/// Error that can occur when constructing a `Triangle`
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// At least two points are identical to each other
    IdenticalPoints,

    /// The three points are on a line
    PointsOnLine,
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use crate::math::Point;

    use super::{Error, Polygon};

    #[test]
    fn validation() {
        let triangle =
            Polygon::new([point![0., 0.], point![0., 1.], point![1., 1.]]);
        let points_on_a_line =
            Polygon::new([point![0., 0.], point![1., 1.], point![2., 2.]]);
        let collapsed_points =
            Polygon::new([point![0., 0.], point![1., 1.], point![1., 1.]]);

        assert!(triangle.is_ok());
        assert_eq!(points_on_a_line, Err(Error::PointsOnLine));
        assert_eq!(collapsed_points, Err(Error::IdenticalPoints));
    }

    #[test]
    fn normalization() {
        let a = point![0., 0.];
        let b = point![0., 1.];
        let c = point![1., 1.];

        // Test with triangles in both directions, to make sure the
        // normalization preserves direction.
        test(a, b, c);
        test(a, c, b);

        fn test<const D: usize>(a: Point<D>, b: Point<D>, c: Point<D>) {
            let abc = Polygon::new([a, b, c]).unwrap();
            let bca = Polygon::new([b, c, a]).unwrap();
            let cab = Polygon::new([c, a, b]).unwrap();

            assert_eq!(abc.points(), bca.points());
            assert_eq!(abc.points(), cab.points());

            // But don't change order of triangle points.
            assert!(
                abc.points() == [a, b, c]
                    || abc.points() == [b, c, a]
                    || abc.points() == [c, b, a]
            );
        }
    }
}