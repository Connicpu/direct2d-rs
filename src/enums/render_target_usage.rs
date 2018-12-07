#[enum_flags(u32)]
pub enum RenderTargetUsage {
    FORCE_BITMAP_REMOTING = 0x1,
    GDI_COMPATIBLE = 0x2,
}
