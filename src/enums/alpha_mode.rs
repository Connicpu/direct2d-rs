#[auto_enum::auto_enum(u32, checked)]
pub enum AlphaMode {
    Unknown = 0,
    Premultiplied = 1,
    Straight = 2,
    Ignore = 3,
}
