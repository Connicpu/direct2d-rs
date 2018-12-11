use com_wrapper::ComWrapper;
use direct2d::brush::SolidColorBrush;
use direct2d::enums::{BitmapOptions,  FigureBegin::*, FigureEnd::*, };
use direct2d::geometry::Path;
use direct2d::image::Bitmap;
use direct2d::{Device, DeviceContext, };
use direct3d11::enums::{BindFlags, CpuAccessFlags, CreateDeviceFlags, Usage};
use dxgi::enums::{Format, MapFlags};
use math2d::{Matrix3x2f, Point2f};

const TEXTURE_WIDTH: u32 = 2048;
const TEXTURE_HEIGHT: u32 = 2048;
const DPI: f32 = 96.0 * TEXTURE_HEIGHT as f32 / 100.0;

const TEXTURE_WIDTH_S: usize = TEXTURE_WIDTH as usize;
const TEXTURE_HEIGHT_S: usize = TEXTURE_HEIGHT as usize;

fn main() {
    // Create the D2D factory
    let d2d = direct2d::Factory::new().unwrap();

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
    let target = Bitmap::create(&context)
        .with_dxgi_surface(&tex.as_dxgi())
        .with_dpi(DPI, DPI)
        .with_options(BitmapOptions::TARGET)
        .build()
        .unwrap();

    // Black brush for the main text
    let fg_brush = SolidColorBrush::create(&context)
        .with_color(0x00_00_00)
        .build()
        .unwrap();
    let bg_brush = SolidColorBrush::create(&context)
        .with_color(math2d::Color::RED)
        .build()
        .unwrap();

    let xo = 0.25;
    let yo = 0.4330127018922193;

    // Create a hexagon
    let hex = Path::create(&d2d).unwrap()
        .with_line_figure(Filled, Closed, &[
            (100.0 * 0.5, 100.0 * 0.0).into(),
            (100.0 * xo, 100.0 * yo).into(),
            (100.0 * -xo, 100.0 * yo).into(),
            (100.0 * -0.5, 100.0 * 0.0).into(),
            (100.0 * -xo, 100.0 * -yo).into(),
            (100.0 * xo, 100.0 * -yo).into(),
        ])
        .finish()
        .unwrap();

    // Start drawing to the texture
    context.set_target(&target);
    context.set_dpi(DPI, DPI);
    context.begin_draw();

    // Make the background clear
    context.clear((0x00_00_00, 0.0));
    
    // Draw the hexagon
    let transform = Matrix3x2f::scaling(99.0 / 100.0, Point2f::ORIGIN)
                  * Matrix3x2f::translation([50.0, 50.0]);
    context.set_transform(&transform);
    context.fill_geometry(&hex, &bg_brush);
    context.draw_geometry(&hex, &fg_brush, 1.0, None);

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

    println!("Saving image...");
    image::save_buffer(
        "hexagon.png",
        &raw_pixels,
        TEXTURE_WIDTH,
        TEXTURE_HEIGHT,
        image::ColorType::RGBA(8),
    ).unwrap();
}
