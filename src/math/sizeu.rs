use winapi::um::d2d1::D2D1_SIZE_U;

#[derive(Copy, Clone)]
#[repr(C)]
/// 2D integer size (width, height).
pub struct SizeU(pub D2D1_SIZE_U);

math_wrapper!(SizeU: D2D1_SIZE_U);
