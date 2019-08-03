#[auto_enum::auto_enum(u32, checked)]
pub enum ExtendMode {
    Clamp = 0,
    Wrap = 1,
    Mirror = 2,
}
