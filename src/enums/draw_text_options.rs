#[enum_flags(u32)]
pub enum DrawTextOptions {
    NO_SNAP = 0x1,
    CLIP = 0x2,
    ENABLE_COLOR_FONT = 0x4,
}
