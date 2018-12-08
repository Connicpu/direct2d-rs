#[enum_flags(u32)]
pub enum BitmapOptions {
    NONE = 0,
    TARGET = 0x1,
    CANNOT_DRAW = 0x2,
    CPU_READ = 0x4,
    GDI_COMPATIBLE = 0x8,
}
