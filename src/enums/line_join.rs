#[auto_enum(u32, checked)]
pub enum LineJoin {
    Miter = 0,
    Bevel = 1,
    Round = 2,
    MiterOrBevel = 3,
}
