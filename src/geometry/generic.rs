use enums::GeometryType;
use geometry::{Ellipse, Group, Path, Rectangle, RoundedRectangle, Transformed};

use winapi::um::d2d1::{ID2D1EllipseGeometry, ID2D1Geometry, ID2D1GeometryGroup,
                       ID2D1RectangleGeometry, ID2D1RoundedRectangleGeometry,
                       ID2D1TransformedGeometry};
use winapi::um::d2d1_1::ID2D1PathGeometry1;
use wio::com::ComPtr;

#[repr(C)]
#[derive(Clone)]
pub struct GenericGeometry {
    ptr: ComPtr<ID2D1Geometry>,
}

impl GenericGeometry {
    pub fn get_geometry_type(&self) -> GeometryType {
        if self.ptr.cast::<ID2D1TransformedGeometry>().is_ok() {
            return GeometryType::Transformed;
        }
        if self.ptr.cast::<ID2D1GeometryGroup>().is_ok() {
            return GeometryType::Group;
        }
        if self.ptr.cast::<ID2D1PathGeometry1>().is_ok() {
            return GeometryType::Path;
        }
        if self.ptr.cast::<ID2D1RoundedRectangleGeometry>().is_ok() {
            return GeometryType::RoundedRectangle;
        }
        if self.ptr.cast::<ID2D1RectangleGeometry>().is_ok() {
            return GeometryType::Rectangle;
        }
        if self.ptr.cast::<ID2D1EllipseGeometry>().is_ok() {
            return GeometryType::Ellipse;
        }

        GeometryType::Unknown
    }

    #[inline]
    pub fn as_ellipse(&self) -> Option<Ellipse> {
        Some(unsafe { Ellipse::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_group(&self) -> Option<Group> {
        Some(unsafe { Group::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_path(&self) -> Option<Path> {
        Some(unsafe { Path::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_rectangle(&self) -> Option<Rectangle> {
        Some(unsafe { Rectangle::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_rounded_rectangle(&self) -> Option<RoundedRectangle> {
        Some(unsafe { RoundedRectangle::from_ptr(self.ptr.cast().ok()?) })
    }

    #[inline]
    pub fn as_transformed(&self) -> Option<Transformed> {
        Some(unsafe { Transformed::from_ptr(self.ptr.cast().ok()?) })
    }
}

geometry_type!(GenericGeometry: ID2D1Geometry);
