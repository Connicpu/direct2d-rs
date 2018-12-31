use dcommon::Error;
use direct2d::brush::SolidColorBrush;
use direct2d::enums::{BitmapOptions, DrawTextOptions};
use direct2d::image::Bitmap1;
use direct2d::{Device, DeviceContext};
use direct3d11::enums::{BindFlags, CreateDeviceFlags};
use directwrite::{TextFormat, TextLayout};
use dxgi::enums::Format;

#[test]
fn draw_to_texture() {
    const TEXTURE_WIDTH: u32 = 512;
    const TEXTURE_HEIGHT: u32 = 128;
    const DPI: f32 = 2.0;

    // Create the DWrite and D2D factories
    let dwrite = directwrite::Factory::new().unwrap();
    let d2d = direct2d::factory::Factory1::new().unwrap();

    // Initialize a D3D Device
    let (_, d3d, _) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    // Create the D2D Device and Context
    let device = Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let mut context = DeviceContext::create(&device).unwrap();

    // Create a texture to render to
    let tex = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .with_format(Format::B8G8R8A8Unorm)
        .with_bind_flags(BindFlags::RENDER_TARGET | BindFlags::SHADER_RESOURCE)
        .build()
        .unwrap();

    // Bind the backing texture to a D2D Bitmap
    let target = Bitmap1::create(&context)
        .with_dxgi_surface(&tex.as_dxgi())
        .with_dpi(96.0 * DPI, 96.0 * DPI)
        .with_options(BitmapOptions::TARGET)
        .build()
        .unwrap();

    // Get the Segoe UI font
    let font = TextFormat::create(&dwrite)
        .with_family("Segoe UI")
        .with_size(16.0)
        .build()
        .unwrap();

    // Lay out our testing text, which contains an emoji
    let text = TextLayout::create(&dwrite)
        .with_str("Testing testing! \u{1F604}")
        .with_format(&font)
        .with_size(TEXTURE_WIDTH as f32 / DPI, TEXTURE_HEIGHT as f32 / DPI)
        .build()
        .unwrap();

    // Black brush for the main text
    let brush = SolidColorBrush::create(&context)
        .with_color(0x00_00_00)
        .build()
        .unwrap();

    // Start drawing to the texture
    context.begin_draw();
    context.set_target(&target);

    // Make the background white
    context.clear(0xFF_FF_FF);

    // Draw the text
    context.draw_text_layout(
        (0.0, 0.0),
        &text,
        &brush,
        DrawTextOptions::ENABLE_COLOR_FONT,
    );

    // Finish
    context.end_draw().unwrap();

    // Everything uses proper wrappers so all of our resources are destroyed
}

#[test]
fn fail_bad_bitmap_formats() {
    let d2d = direct2d::factory::Factory1::new().unwrap();

    // Initialize a D3D Device
    let (_, d3d, _) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    // Create the D2D Device and Context
    let device = Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let context = DeviceContext::create(&device).unwrap();

    // Create a texture
    let tex = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(128, 128)
        .with_format(Format::B8G8R8A8Unorm)
        .with_bind_flags(BindFlags::RENDER_TARGET | BindFlags::SHADER_RESOURCE)
        .build()
        .unwrap();

    let result = Bitmap1::create(&context)
        .with_dxgi_surface(&tex.as_dxgi())
        .with_format(Format::R8G8B8A8Unorm)
        .with_options(BitmapOptions::TARGET)
        .build()
        .map(|_| ());

    assert_eq!(result, Err(Error::INVALIDARG));
}
