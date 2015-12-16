extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate direct2d;

use std::{ptr, mem};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::*;
use direct2d::{Factory, RenderTarget};
use direct2d::render_target::RenderTargetBacking;
use direct2d::math::ColorF;

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
    fn create_target(self, factory: &mut ID2D1Factory) -> Result<*mut ID2D1RenderTarget, HRESULT> {
        unsafe {
            let mut ptr: *mut ID2D1HwndRenderTarget = ptr::null_mut();
            let hr = factory.CreateHwndRenderTarget(
                &self.props,
                &self.hprops,
                &mut ptr as *mut _,
            );
            
            let ptr: *mut _ = &mut **ptr;
            
            if SUCCEEDED(hr) {
                Ok(ptr)
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
            user32::SetWindowLongPtrW(hwnd, GWLP_USERDATA, mem::transmute(cs.lpCreateParams));
            
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
            let window: &mut Window = mem::transmute(user32::GetWindowLongPtrW(hwnd, GWLP_USERDATA));
            
            let rt = window.target.as_mut().unwrap();
            rt.begin_draw();
            rt.clear(&ColorF::uint_rgb(0xFF0000, 1.0));
            rt.end_draw().unwrap();
            
            user32::DefWindowProcW(hwnd, msg, wp, lp)
        },
        WM_TIMER => {
            user32::DestroyWindow(hwnd);
            
            0
        },
        _ => user32::DefWindowProcW(hwnd, msg, wp, lp)
    }
}

unsafe fn real_window_test() {
    let factory = Factory::create().unwrap();
    
    let mut window = Window {
        hwnd: ptr::null_mut(),
        factory: factory,
        target: None,
    };
    
    let hinst: HINSTANCE = kernel32::GetModuleHandleW(ptr::null());
    let class_name = "Test D2D1 Window Class".to_wide_null();
    let window_name = "Test D2D1 Window".to_wide_null();
    
    user32::RegisterClassW(&WNDCLASSW {
        style: CS_DBLCLKS | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        cbWndExtra: mem::size_of::<&mut Window>() as c_int,
        hInstance: hinst,
        lpszClassName: class_name.as_ptr(),
        
        .. mem::zeroed()
    });
    
    let hwnd = user32::CreateWindowExW(
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
    
    user32::ShowWindow(hwnd, SW_SHOW);
    user32::SetTimer(hwnd, 0, 1000, None);
    
    let mut msg: MSG = mem::uninitialized();
    loop {
        match user32::GetMessageW(&mut msg, hwnd, 0, 0) {
            -1 => break,
            _ => {
                user32::TranslateMessage(&msg);
                user32::DispatchMessageW(&msg);
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


