#![feature(const_fn)]

extern crate winapi;
extern crate direct2d;

use std::{ptr, mem};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use direct2d::{Factory, RenderTarget};
use direct2d::render_target::RenderTargetBacking;
use direct2d::math::*;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::shared::winerror::*;
use winapi::shared::dxgiformat::*;
use winapi::um::dcommon::*;
use winapi::um::d2d1::*;
use winapi::um::d2d1_1::*;
use winapi::um::winuser::*;
use winapi::um::libloaderapi::*;

pub const BACKGROUND: ColorF = ColorF::uint_rgb(0x2A14CC, 1.0);
pub const HIGHLIGHT: ColorF = ColorF::uint_rgb(0x483D99, 1.0);
pub const ACCENT: ColorF = ColorF::uint_rgb(0x006AFF, 1.0);
pub const FOREGROUND: ColorF = ColorF::uint_rgb(0xFFA940, 1.0);
pub const FADED: ColorF = ColorF::uint_rgb(0xCC5E14, 1.0);

fn paint_window(window: &mut Window) {
    let rt = window.target.as_mut().unwrap();
    
    let accent_brush = rt.create_solid_color_brush(&ACCENT, &BrushProperties::default()).unwrap();
    let foreground_brush = rt.create_solid_color_brush(&FOREGROUND, &BrushProperties::default()).unwrap();
        
    rt.begin_draw();
    rt.clear(&BACKGROUND);
    
    rt.fill_rectangle(&RectF::new(50.0, 50.0, 750.0, 430.0), &accent_brush);
    rt.fill_rectangle(&RectF::new(150.0, 150.0, 650.0, 330.0), &foreground_brush);
    
    rt.end_draw().unwrap();
}

struct Window {
    hwnd: HWND,
    factory: Factory,
    target: Option<RenderTarget>,
}

struct WindowCreate {
    props: D2D1_RENDER_TARGET_PROPERTIES,
    hprops: D2D1_HWND_RENDER_TARGET_PROPERTIES,
}

unsafe impl RenderTargetBacking for WindowCreate {
    fn create_target(self, factory: &mut ID2D1Factory1) -> Result<*mut ID2D1RenderTarget, HRESULT> {
        unsafe {
            let mut ptr: *mut ID2D1HwndRenderTarget = ptr::null_mut();
            let hr = factory.CreateHwndRenderTarget(
                &self.props,
                &self.hprops,
                &mut ptr as *mut _,
            );
            
            if SUCCEEDED(hr) {
                Ok(ptr as *mut _)
            } else {
                Err(From::from(hr))
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Set the window pointer into the creation parameters
            let cs: &CREATESTRUCTW = mem::transmute(lp);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, mem::transmute(cs.lpCreateParams));
            
            // Create the direct2d stuff
            let params = WindowCreate {
                props: D2D1_RENDER_TARGET_PROPERTIES {
                    _type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                    pixelFormat: D2D1_PIXEL_FORMAT {
                        format: DXGI_FORMAT_UNKNOWN,
                        alphaMode: D2D1_ALPHA_MODE_UNKNOWN,
                    },
                    dpiX: 0.0,
                    dpiY: 0.0,
                    usage: D2D1_RENDER_TARGET_USAGE_NONE,
                    minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
                },
                hprops: D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd: hwnd,
                    pixelSize: D2D1_SIZE_U {
                        width: 800,
                        height: 480,
                    },
                    presentOptions: D2D1_PRESENT_OPTIONS_NONE,
                },
            };
            
            let window: &mut Window = mem::transmute(cs.lpCreateParams);
            window.target = Some(window.factory.create_render_target(params).unwrap());
            
            0
        },
        WM_PAINT => {
            let window: &mut Window = mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
            paint_window(window);
            
            DefWindowProcW(hwnd, msg, wp, lp)
        },
        WM_TIMER => {
            DestroyWindow(hwnd);
            
            0
        },
        _ => DefWindowProcW(hwnd, msg, wp, lp)
    }
}

unsafe fn real_window_test() {
    let factory = Factory::new().unwrap();
    
    let mut window = Window {
        hwnd: ptr::null_mut(),
        factory: factory,
        target: None,
    };
    
    let hinst: HINSTANCE = GetModuleHandleW(ptr::null());
    let class_name = "Test D2D1 Window Class".to_wide_null();
    let window_name = "Test D2D1 Window".to_wide_null();
    
    RegisterClassW(&WNDCLASSW {
        style: CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        cbWndExtra: mem::size_of::<&mut Window>() as c_int,
        hInstance: hinst,
        lpszClassName: class_name.as_ptr(),
        
        .. mem::zeroed()
    });
    
    let hwnd = CreateWindowExW(
        0, // dwExStyle
        class_name.as_ptr(),
        window_name.as_ptr(),
        WS_OVERLAPPED,
        CW_USEDEFAULT, // x
        CW_USEDEFAULT, // y
        800, // width
        480, // height
        ptr::null_mut(),
        ptr::null_mut(),
        hinst,
        &mut window as *mut _ as LPVOID,
    );
    
    assert!(hwnd != ptr::null_mut());
    window.hwnd = hwnd;
    
    ShowWindow(hwnd, SW_SHOW);
    SetTimer(hwnd, 0, 1000, None);
    
    let mut msg: MSG = mem::uninitialized();
    loop {
        match GetMessageW(&mut msg, hwnd, 0, 0) {
            -1 => break,
            _ => {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

#[test]
fn window_test() {
    unsafe { real_window_test() };
}

pub trait ToWide { 
    fn to_wide(&self) -> Vec<u16>; 
    fn to_wide_null(&self) -> Vec<u16>; 
} 

impl<T> ToWide for T where T: AsRef<OsStr> { 
    fn to_wide(&self) -> Vec<u16> { 
        self.as_ref().encode_wide().collect()
    } 
    fn to_wide_null(&self) -> Vec<u16> { 
        self.as_ref().encode_wide().chain(Some(0)).collect() 
    } 
}


