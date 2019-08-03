#[auto_enum::auto_enum(u32, checked)]
pub enum AntialiasMode {
    PerPrimitive = 0,
    Aliased = 1,
}
