#[enum_flags(u32)]
pub enum PresentOptions {
    RETAIN_CONTENTS = 0x1,
    IMMEDIATELY = 0x2,
}
