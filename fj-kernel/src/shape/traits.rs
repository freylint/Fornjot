use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{
    validate::Validatable, Geometry, Shape, Topology, ValidationResult,
};

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

/// Marker trait for topological objects
pub trait TopoObject: Validatable + Sized + topo::Sealed {
    /// Internal function
    ///
    /// Use [`Topology::merge`] instead.
    fn merge_into(
        &self,
        geometry: &mut Geometry,
        topology: &mut Topology,
    ) -> ValidationResult<Self>;
}

impl topo::Sealed for Vertex {}
impl topo::Sealed for Edge {}
impl topo::Sealed for Cycle {}
impl topo::Sealed for Face {}

impl TopoObject for Vertex {
    fn merge_into(
        &self,
        geometry: &mut Geometry,
        topology: &mut Topology,
    ) -> ValidationResult<Self> {
        if geometry.handle_for(&self.point()).is_none() {
            let point = geometry.add_point(self.point());
            return topology.add_vertex(Vertex { point });
        }

        todo!()
    }
}

impl TopoObject for Edge {
    fn merge_into(
        &self,
        geometry: &mut Geometry,
        topology: &mut Topology,
    ) -> ValidationResult<Self> {
        todo!()
    }
}

impl TopoObject for Cycle {
    fn merge_into(
        &self,
        geometry: &mut Geometry,
        topology: &mut Topology,
    ) -> ValidationResult<Self> {
        todo!()
    }
}

impl TopoObject for Face {
    fn merge_into(
        &self,
        geometry: &mut Geometry,
        topology: &mut Topology,
    ) -> ValidationResult<Self> {
        todo!()
    }
}

mod topo {
    pub trait Sealed {}
}
