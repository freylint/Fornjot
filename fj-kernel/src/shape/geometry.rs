use anymap::AnyMap;
use fj_math::{Point, Transform};

use crate::{
    geometry::{Curve, Surface},
    shape::Store,
    topology::Face,
};

use super::{
    handle::{Handle, Storage},
    Curves, Faces, Iter, GeoObject, Points, Surfaces,
};

/// API to access a shape's geometry
///
/// Other than topology, geometry doesn't need to be validated. Hence adding
/// geometry is infallible.
///
/// There are several reasons for this:
/// - Geometry doesn't refer to other objects, so structural validation doesn't
///   apply.
/// - There simply no reason that geometry needs to be unique. In addition, it's
///   probably quite hard to rule out generating duplicate geometry. Think about
///   line segment edges that are on identical lines, but are created
///   separately.
/// - Geometric validation doesn't apply either. It simply doesn't matter, if
///   curves or surfaces intersect, for example, as long as they don't do that
///   where an edge or face is defined.
pub struct Geometry<'r> {
    pub(super) points: &'r mut Points,
    pub(super) curves: &'r mut Curves,
    pub(super) surfaces: &'r mut Surfaces,

    // This is needed here for a weird workaround, which in turn is necessary
    // because triangle representation still exists. Once triangle
    // representation is no longer a thing, this field can be moved to
    // `Topology`, where it belongs.
    //
    // This issue has some context on triangle representation:
    // https://github.com/hannobraun/Fornjot/issues/97
    pub(super) faces: &'r mut Faces,
}

impl Geometry<'_> {
    /// Add a point to the shape
    pub fn add_point(&mut self, point: Point<3>) -> Handle<Point<3>> {
        let storage = Storage::new(point);
        let handle = storage.handle();

        self.points.push(storage);

        handle
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        let storage = Storage::new(curve);
        let handle = storage.handle();

        self.curves.push(storage);

        handle
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        let storage = Storage::new(surface);
        let handle = storage.handle();

        self.surfaces.push(storage);

        handle
    }

    /// Transform the geometry of the shape
    ///
    /// Since the topological types refer to geometry, and don't contain any
    /// geometry themselves, this transforms the whole shape.
    pub fn transform(&mut self, transform: &Transform) {
        for point in self.points.iter_mut() {
            let trans = {
                let point = point.get();
                transform.transform_point(&point)
            };
            *point.get_mut() = trans;
        }
        for curve in self.curves.iter_mut() {
            let trans = {
                let curve = curve.get();
                curve.transform(transform)
            };
            *curve.get_mut() = trans;
        }
        for surface in self.surfaces.iter_mut() {
            let trans = {
                let surface = surface.get();
                surface.transform(transform)
            };
            *surface.get_mut() = trans;
        }

        // While some faces use triangle representation, we need this weird
        // workaround here.
        for face in self.faces.iter_mut() {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.get_mut().deref_mut() {
                for triangle in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        }
    }

    /// Access an iterator over all points
    ///
    /// The caller must not make any assumptions about the order of points.
    pub fn points(&self) -> Iter<Point<3>> {
        Iter::new(self.points)
    }

    /// Access an iterator over all curves
    ///
    /// The caller must not make any assumptions about the order of curves.
    pub fn curves(&self) -> Iter<Curve> {
        Iter::new(self.curves)
    }

    /// Access an iterator over all surfaces
    ///
    /// The caller must not make any assumptions about the order of surfaces.
    pub fn surfaces(&self) -> Iter<Surface> {
        Iter::new(self.surfaces)
    }

    /// Access handle for a geometric object
    ///
    /// Returns the handle that refers to the given geometric object, if it is
    /// part of the shape. Returns `None`, if it isn't.
    ///
    /// # Implementation note
    ///
    /// If `object` is present multiple times, the handle of the first that is
    /// found is returned. This is weird. It would be better, if geometric
    /// objects were unique.
    pub fn handle_for<T>(&self, object: &T) -> Option<Handle<T>>
    where
        T: 'static + GeoObject,
    {
        let mut map = AnyMap::new();

        // Cloning the collections is a bit unfortunate, but unless that turns
        // into a real performance issue, it's probably fine.
        //
        // What's important is, that this method can be implemented, which this
        // placeholder here proves. If necessary, the implementation can be
        // optimized using a different approach.
        map.insert(self.points.clone());
        map.insert(self.curves.clone());
        map.insert(self.surfaces.clone());

        map.get::<Store<T>>()
            // Can't panic, as `T` is bound by `Object`, and we added the stores
            // for all geometric objects above.
            .unwrap()
            .iter()
            .find(|obj| &*obj.get() == object)
            .map(|storage| storage.handle())
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        geometry::{Curve, Surface},
        shape::Shape,
    };

    #[test]

    fn handle_for() {
        let mut shape = Shape::new();

        let point = Point::origin();
        let curve = Curve::x_axis();
        let surface = Surface::x_y_plane();

        assert!(shape.geometry().handle_for(&point).is_none());
        assert!(shape.geometry().handle_for(&curve).is_none());
        assert!(shape.geometry().handle_for(&surface).is_none());

        shape.geometry().add_point(point);
        shape.geometry().add_curve(curve);
        shape.geometry().add_surface(surface);

        assert!(shape.geometry().handle_for(&point).is_some());
        assert!(shape.geometry().handle_for(&curve).is_some());
        assert!(shape.geometry().handle_for(&surface).is_some());
    }
}
