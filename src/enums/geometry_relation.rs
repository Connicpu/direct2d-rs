#[auto_enum::auto_enum(u32, checked)]
pub enum GeometryRelation {
    Unknown = 0,
    Disjoint = 1,
    IsContained = 2,
    Contains = 3,
    Overlap = 4,
}
