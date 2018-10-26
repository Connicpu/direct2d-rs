extern crate direct2d;
extern crate direct3d11;
extern crate dxgi;

use direct2d::image::Bitmap;
use direct2d::{Device, DeviceContext, Factory};
use direct3d11::flags::CreateDeviceFlags;
use dxgi::Format;

#[test]
fn empty_bitmap() {
    let d2d = Factory::new().unwrap();
    let (_, d3d, _) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();
    let device = Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let context = DeviceContext::create(&device, false).unwrap();

    Bitmap::create(&context)
        .with_blank_image((64, 64))
        .with_format(Format::R8G8B8A8Unorm)
        .build()
        .unwrap();
}
