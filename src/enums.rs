use std::marker::PhantomData;

pub trait CheckedEnum: Sized + Copy + 'static {
    fn to_u32(self) -> u32;
    fn from_u32(value: u32) -> Option<Self>;
}

d2d_enums! {
    pub enum ExtendMode {
        Clamp = 0,
        Wrap = 1,
        Mirror = 2,
    }

    pub enum GeometryRelation {
        Unknown = 0,
        Disjoint = 1,
        IsContained = 2,
        Contains = 3,
        Overlap = 4,
    }

    pub enum FillMode {
        Alternate = 0,
        Winding = 1,
    }

    pub enum FigureBegin {
        Filled = 0,
        Hollow = 1,
    }

    pub enum FigureEnd {
        Open = 0,
        Closed = 1,
    }

    pub enum PathSegment {
        None = 0,
        ForceUnstroked = 1,
        ForceRoundLineJoin = 2,
    }

    pub enum Gamma {
        _2_2 = 0,
        _1_0 = 1,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UncheckedEnum<T: CheckedEnum> {
    pub value: u32,
    _marker: PhantomData<T>,
}

impl<T> UncheckedEnum<T>
where
    T: CheckedEnum,
{
    pub fn new(value: u32) -> Self {
        UncheckedEnum {
            value,
            _marker: PhantomData,
        }
    }

    pub fn as_enum(self) -> Option<T> {
        T::from_u32(self.value)
    }
}

pub enum GeometryType {
    Unknown,
    Ellipse,
    Group,
    Path,
    Rectangle,
    RoundedRectangle,
    Transformed,
}
