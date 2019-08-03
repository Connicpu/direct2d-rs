extern crate direct2d;
extern crate direct3d11;
extern crate math2d;

use direct2d::brush::SolidColorBrush;
use direct2d::device::Device;
use direct2d::device_context::DeviceContext;
use direct2d::factory::Factory1;
use direct3d11::enums::CreateDeviceFlags;
use math2d::*;

#[test]
fn solid_color() {
    let (_factory, context) = make_context();

    for i in 0u32..(16 * 16 * 16) {
        let color = Color {
            r: ((i >> 8) & 0xF) as f32 / 15.0,
            g: ((i >> 4) & 0xF) as f32 / 15.0,
            b: ((i >> 0) & 0xF) as f32 / 15.0,
            a: 1.0,
        };

        let brush = SolidColorBrush::create(&context)
            .with_color(color)
            .build()
            .unwrap();

        let brush_color = brush.color();

        assert_eq!(color, brush_color);
    }
}

fn make_context() -> (Factory1, DeviceContext) {
    let (_, d3d, _) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();
    let factory = Factory1::new().unwrap();
    let dev = Device::create(&factory, &d3d.as_dxgi()).unwrap();
    let ctx = DeviceContext::create(&dev).unwrap();
    (factory, ctx)
}
