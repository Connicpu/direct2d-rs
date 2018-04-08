extern crate direct2d;
extern crate directwrite;
extern crate dxgi;
extern crate winapi;

use direct2d::RenderTarget;

use std::ptr;

use winapi::Interface;
use winapi::shared::dxgi::{IDXGIDevice, IDXGISurface};
use winapi::shared::dxgitype::DXGI_SAMPLE_DESC;
use winapi::um::d3d11::D3D11_TEXTURE2D_DESC;

#[test]
fn draw_to_texture() {
    let dw_factory = directwrite::Factory::new().unwrap();
    let d2d_factory = direct2d::Factory::new().unwrap();

    let mut d3d_device = ptr::null_mut();
    let mut d3d_devctx = ptr::null_mut();
    let mut d3d_texture = ptr::null_mut();
    let dxgi_device;
    let dxgi_surface;
    unsafe {
        let mut features = 0;
        let hr = winapi::um::d3d11::D3D11CreateDevice(
            ptr::null_mut(),
            1,
            ptr::null_mut(),
            0x20,
            (&[0xb000]).as_ptr(),
            1,
            7,
            &mut d3d_device,
            &mut features,
            &mut d3d_devctx,
        );
        if hr < 0 {
            panic!("D3D11CreateDevice: 0x{:x}", hr);
        }

        let hr = (*d3d_device).CreateTexture2D(
            &D3D11_TEXTURE2D_DESC {
                Width: 320,
                Height: 240,
                MipLevels: 1,
                ArraySize: 1,
                Format: 91,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: 0,
                BindFlags: 32 | 8,
                CPUAccessFlags: 0,
                MiscFlags: 0,
            },
            ptr::null(),
            &mut d3d_texture,
        );
        if hr < 0 {
            panic!("ID3D11Device::CreateTexture2D: 0x{:x}", hr);
        }

        let mut dxgi_ptr = ptr::null_mut();
        let hr =
            (*d3d_device).QueryInterface(&IDXGIDevice::uuidof(), &mut dxgi_ptr as *mut _ as *mut _);
        if hr < 0 {
            panic!("ID3D11Device::QueryInterface(IDXGIDevice): 0x{:x}", hr);
        }
        dxgi_device = dxgi::device::Device::from_raw(dxgi_ptr);

        let mut dxgi_ptr = ptr::null_mut();
        let hr = (*d3d_texture)
            .QueryInterface(&IDXGISurface::uuidof(), &mut dxgi_ptr as *mut _ as *mut _);
        if hr < 0 {
            panic!("ID3D11Texture2D::QueryInterface(IDXGISurface): 0x{:x}", hr);
        }
        dxgi_surface = dxgi::surface::Surface::from_raw(dxgi_ptr);
    }

    let d2d_device = d2d_factory.create_device(&dxgi_device).unwrap();
    let mut d2d_devctx = d2d_device.create_device_context().unwrap();

    let font = dw_factory
        .create(
            directwrite::text_format::ParamBuilder::new()
                .family("Segoe UI")
                .size(16.0)
                .build()
                .unwrap(),
        )
        .unwrap();

    let text = dw_factory
        .create(
            directwrite::text_layout::ParamBuilder::new()
                .text("Testing testing!")
                .font(font)
                .width(320.0)
                .height(240.0)
                .build()
                .unwrap(),
        )
        .unwrap();

    let d2d_surface = d2d_devctx
        .create_bitmap_from_dxgi_surface((96.0, 96.0), true, &dxgi_surface)
        .unwrap();

    let d2d_brush = d2d_devctx
        .create_solid_color_brush(0x0000FF, &Default::default())
        .unwrap();

    d2d_devctx.set_target(&d2d_surface);
    d2d_devctx.begin_draw();
    d2d_devctx.clear(&0.into());
    d2d_devctx.draw_text_layout(&(0.0, 0.0).into(), &text, &d2d_brush, &[]);
    d2d_devctx.end_draw().unwrap();

    unsafe {
        (*d3d_texture).Release();
        (*d3d_devctx).Release();
        (*d3d_device).Release();
    }
}
