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

    pub enum RenderTargetType {
        Default = 0,
        Software = 1,
        Hardware = 2,
    }

    pub enum AlphaMode {
        Unknown = 0,
        Premultiplied = 1,
        Straight = 2,
        Ignore = 3,
    }

    pub enum FeatureLevel {
        Default = 0,
        Level9 = 37120,
        Level10 = 40960,
    }

    pub enum SweepDirection {
        CounterClockwise = 0,
        Clockwise = 1,
    }

    pub enum ArcSize {
        Small = 0,
        Large = 1,
    }

    pub enum CapStyle {
        Flat = 0,
        Square = 1,
        Round = 2,
        Triangle = 3,
    }

    pub enum LineJoin {
        Miter = 0,
        Bevel = 1,
        Round = 2,
        MiterOrBevel = 3,
    }

    pub enum DashStyle {
        Solid = 0,
        Dash = 1,
        Dot = 2,
        DashDot = 3,
        DashDotDot = 4,
        Custom = 5,
    }

    pub enum StrokeTransformType {
        Normal = 0,
        Fixed = 1,
        Hairline = 2,
    }

    pub enum BitmapInterpolationMode {
        NearestNeighbor = 0,
        Linear = 1,
    }

    pub enum AntialiasMode {
        PerPrimitive = 0,
        Aliased = 1,
    }

    pub enum LayerOptions {
        None = 0,
        InitializeForCleartype = 1,
    }
}

d2d_flags! {
    #[repr(u32)]
    pub enum RenderTargetUsage {
        FORCE_BITMAP_REMOTING = 0x1,
        GDI_COMPATIBLE = 0x2,
    }

    #[repr(u32)]
    pub enum PresentOptions {
        RETAIN_CONTENTS = 0x1,
        IMMEDIATELY = 0x2,
    }

    #[repr(u32)]
    pub enum DrawTextOptions {
        NO_SNAP = 0x1,
        CLIP = 0x2,
        ENABLE_COLOR_FONT = 0x4,
    }

    #[repr(u32)]
    pub enum BitmapOptions {
        TARGET = 0x1,
        CANNOT_DRAW = 0x2,
        CPU_READ = 0x4,
        GDI_COMPATIBLE = 0x8,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// An enum value which was obtained from FFI and therefore could potentially be
/// out of range. It's up to you to decide when/if to validate it.
pub struct UncheckedEnum<T: CheckedEnum> {
    pub value: u32,
    _marker: PhantomData<T>,
}

impl<T> UncheckedEnum<T>
where
    T: CheckedEnum,
{
    #[inline]
    pub fn new(value: u32) -> Self {
        UncheckedEnum {
            value,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_enum(self) -> Option<T> {
        T::from_u32(self.value)
    }

    #[inline]
    pub unsafe fn as_enum_unchecked(self) -> T {
        assert_eq!(::std::mem::size_of_val(&self), ::std::mem::size_of::<T>());
        ::std::mem::transmute_copy(&self)
    }
}

impl<T> From<u32> for UncheckedEnum<T>
where
    T: CheckedEnum,
{
    fn from(value: u32) -> Self {
        UncheckedEnum::new(value)
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
