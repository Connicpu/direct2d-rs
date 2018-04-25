#![feature(const_fn)]

#[macro_use]
extern crate lazy_static;

extern crate direct2d;
extern crate winapi;
extern crate wio;

use direct2d::brush::SolidColorBrush;
use direct2d::math::*;
use direct2d::render_target::HwndRenderTarget;
use direct2d::{Factory, RenderTarget};
use std::{mem, ptr};

use winapi::ctypes::c_int;
use winapi::shared::minwindef::*;
use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;
use wio::wide::ToWide;

lazy_static! {
    pub static ref BACKGROUND: ColorF = ColorF::uint_rgb(0x2A14CC, 1.0);
    pub static ref HIGHLIGHT: ColorF = ColorF::uint_rgb(0x483D99, 1.0);
    pub static ref ACCENT: ColorF = ColorF::uint_rgb(0x006AFF, 1.0);
    pub static ref FOREGROUND: ColorF = ColorF::uint_rgb(0xFFA940, 1.0);
    pub static ref FADED: ColorF = ColorF::uint_rgb(0xCC5E14, 1.0);
}

fn paint_window(window: &mut Window) {
    let rt = window.target.as_mut().unwrap();

    let accent_brush = SolidColorBrush::create(&rt)
        .with_color(*ACCENT)
        .build()
        .unwrap();
    let foreground_brush = SolidColorBrush::create(&rt)
        .with_color(*FOREGROUND)
        .build()
        .unwrap();

    rt.begin_draw();
    rt.clear(*BACKGROUND);

    rt.fill_rectangle((50.0, 50.0, 750.0, 430.0), &accent_brush);
    rt.fill_rectangle((150.0, 150.0, 650.0, 330.0), &foreground_brush);

    rt.end_draw().unwrap();
}

struct Window {
    hwnd: HWND,
    factory: Factory,
    target: Option<HwndRenderTarget>,
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            // Set the window pointer into the creation parameters
            let cs: &CREATESTRUCTW = mem::transmute(lp);
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, mem::transmute(cs.lpCreateParams));
            let window: &mut Window = mem::transmute(cs.lpCreateParams);

            // Create the direct2d stuff
            let target = HwndRenderTarget::create(&window.factory)
                .with_hwnd(hwnd)
                .with_pixel_size(800, 480)
                .build()
                .unwrap();

            window.target = Some(target);

            0
        }
        WM_PAINT => {
            let window: &mut Window = mem::transmute(GetWindowLongPtrW(hwnd, GWLP_USERDATA));
            paint_window(window);

            DefWindowProcW(hwnd, msg, wp, lp)
        }
        WM_TIMER => {
            DestroyWindow(hwnd);

            0
        }
        _ => DefWindowProcW(hwnd, msg, wp, lp),
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

        ..mem::zeroed()
    });

    let hwnd = CreateWindowExW(
        0, // dwExStyle
        class_name.as_ptr(),
        window_name.as_ptr(),
        WS_OVERLAPPED,
        CW_USEDEFAULT, // x
        CW_USEDEFAULT, // y
        800,           // width
        480,           // height
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
