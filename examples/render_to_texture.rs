use com_wrapper::ComWrapper;
use direct2d::brush::SolidColorBrush;
use direct2d::device::Device;
use direct2d::device_context::{DeviceContext, IDeviceContext};
use direct2d::enums::{BitmapOptions, DrawTextOptions};
use direct2d::image::Bitmap1;
use direct2d::render_target::IRenderTarget;
use direct3d11::enums::{BindFlags, CpuAccessFlags, CreateDeviceFlags, Usage};
use directwrite::{TextFormat, TextLayout};
use dxgi::enums::{Format, MapFlags};
use dxgi::surface::ISurface;

const TEXTURE_WIDTH: u32 = 400;
const TEXTURE_HEIGHT: u32 = 200;
const DPI: f32 = 2.0;

const TEXTURE_WIDTH_S: usize = TEXTURE_WIDTH as usize;
const TEXTURE_HEIGHT_S: usize = TEXTURE_HEIGHT as usize;

fn main() {
    // Create the DWrite and D2D factories
    let dwrite = directwrite::Factory::new().unwrap();
    let d2d = direct2d::factory::Factory1::new().unwrap();

    // Initialize a D3D Device
    let (_, d3d, d3d_ctx) = direct3d11::device::Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    // Create the D2D Device and Context
    let device = Device::create(&d2d, &d3d.as_dxgi()).unwrap();
    let mut context = DeviceContext::create(&device).unwrap();

    // Create a texture to render to
    let tex = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .with_format(Format::R8G8B8A8Unorm)
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
        .with_size(24.0)
        .build()
        .unwrap();

    // Lay out our testing text, which contains an emoji
    let text = TextLayout::create(&dwrite)
        .with_str("Testing testing! \u{1F604}\u{1F604}\u{1F604}\u{1F604}\u{1F604}")
        .with_format(&font)
        .with_size(
            TEXTURE_WIDTH as f32 / DPI - 30.0,
            TEXTURE_HEIGHT as f32 / DPI - 30.0,
        )
        .build()
        .unwrap();

    // Black brush for the main text
    let fg_brush = SolidColorBrush::create(&context)
        .with_color(0x00_00_00)
        .build()
        .unwrap();
    let bg_brush = SolidColorBrush::create(&context)
        .with_color(0xFF_7F_7F)
        .build()
        .unwrap();

    println!("fg: {:?}", fg_brush.color());
    println!("bg: {:?}", bg_brush.color());

    // Start drawing to the texture
    context.set_target(&target);
    context.set_dpi(96.0 * DPI, 96.0 * DPI);
    context.begin_draw();

    // Make the background white
    context.clear(0xFF_FF_FF.into());

    let rect = [10.0, 10.0, 190.0, 90.0].into();
    context.fill_rectangle(rect, &bg_brush);
    context.draw_rectangle(rect, &fg_brush, 1.0, None);

    // Draw the text
    context.draw_text_layout(
        (15.0, 15.0).into(),
        &text,
        &fg_brush,
        DrawTextOptions::ENABLE_COLOR_FONT,
    );

    // Finish
    context.end_draw().unwrap();

    let temp_texture = direct3d11::texture2d::Texture2D::create(&d3d)
        .with_size(TEXTURE_WIDTH, TEXTURE_HEIGHT)
        .with_format(Format::R8G8B8A8Unorm)
        .with_bind_flags(BindFlags::NONE)
        .with_usage(Usage::Staging)
        .with_cpu_access(CpuAccessFlags::READ)
        .build()
        .unwrap();

    // Get the data so we can write it to a file
    // TODO: Have a safe way to accomplish this :D
    let mut raw_pixels: Vec<u8> = Vec::with_capacity(TEXTURE_WIDTH_S * TEXTURE_HEIGHT_S * 4);
    unsafe {
        let ctx = &*d3d_ctx.get_raw();
        ctx.CopyResource(temp_texture.get_raw() as *mut _, tex.get_raw() as *mut _);
        ctx.Flush();

        let surface = temp_texture.as_dxgi();
        let map = surface.map(MapFlags::READ).unwrap();
        for y in 0..TEXTURE_HEIGHT {
            raw_pixels.extend_from_slice(&map.row(y)[..TEXTURE_WIDTH_S * 4]);
        }
    }

    println!("buffer size: {}", raw_pixels.len());
    image::save_buffer(
        "temp-image.png",
        &raw_pixels,
        TEXTURE_WIDTH,
        TEXTURE_HEIGHT,
        image::ColorType::RGBA(8),
    )
    .unwrap();
}
