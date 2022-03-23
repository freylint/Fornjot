use fj_math::Point;

use crate::geometry::{Curve, Surface};

/// Marker trait for geometric objects
pub trait GeoObject: PartialEq + geo::Sealed {}

impl geo::Sealed for Point<3> {}
impl geo::Sealed for Curve {}
impl geo::Sealed for Surface {}

impl GeoObject for Point<3> {}
impl GeoObject for Curve {}
impl GeoObject for Surface {}

mod geo {
    pub trait Sealed {}
}
