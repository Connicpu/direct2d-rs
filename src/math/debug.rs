use super::*;
use std::fmt::{Debug, Formatter, Result};

impl Debug for Point2F {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "Point({}, {})", self.0.x, self.0.y)
    }
}

impl Debug for Vector2F {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "Vector<{}, {}>", self.0.x, self.0.y)
    }
}

impl Debug for SizeF {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "Size[{}, {}]", self.0.width, self.0.height)
    }
}

impl Debug for RectF {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "Rect[{}, {}, {}, {}]",
            self.0.left, self.0.top, self.0.right, self.0.bottom
        )
    }
}

impl Debug for ThicknessF {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "Thickness[{}, {}, {}, {}]",
            self.0.left, self.0.top, self.0.right, self.0.bottom
        )
    }
}

impl Debug for RoundedRect {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "RoundedRect({:?}, {}, {})",
            RectF(self.0.rect),
            self.0.radiusX,
            self.0.radiusY
        )
    }
}

impl Debug for Ellipse {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "Ellipse({:?}, {}, {})",
            Point2F(self.0.point),
            self.0.radiusX,
            self.0.radiusY
        )
    }
}

impl Debug for ColorF {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "ColorRGBA({}, {}, {}, {})",
            self.0.r, self.0.g, self.0.b, self.0.a
        )
    }
}

impl Debug for Matrix3x2F {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.debug_list()
            .entry(&[self.0.matrix[0][0], self.0.matrix[0][1]])
            .entry(&[self.0.matrix[1][0], self.0.matrix[1][1]])
            .entry(&[self.0.matrix[2][0], self.0.matrix[2][1]])
            .finish()
    }
}
