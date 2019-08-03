#[auto_enum::enum_flags(u32)]
pub enum PresentOptions {
    NONE = 0,
    RETAIN_CONTENTS = 0x1,
    IMMEDIATELY = 0x2,
}
