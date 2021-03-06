//#![windows_subsystem = "windows"]

use std::mem;
use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

mod util;
use crate::util::*;
mod brushes;
use crate::brushes::*;
mod close_button;
use crate::close_button::*;
mod button;
use crate::button::*;
mod progress;
use crate::progress::*;

static mut H_INSTANCE: HINSTANCE = null_mut();

const BTN_NEXT: i32 = 1001;
const BTN_BACK: i32 = 1002;

extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            create_close_button(hwnd);

            let next = create_button(hwnd, BTN_NEXT, "Next", Aurora::Purple);
            let back = create_button(hwnd, BTN_BACK, "Back", Aurora::Orange);
            let progress = create_progress(hwnd, 0.25);

            unsafe {
                let mut rect = default_rect();
                GetClientRect(hwnd, &mut rect);
                
                SetWindowPos(next, HWND_TOP, rect.right - (8+64), rect.bottom - (8+24), 64, 24, SWP_NOZORDER);
                SetWindowPos(back, HWND_TOP, rect.right - (8+64+8+64), rect.bottom - (8+24), 64, 24, SWP_NOZORDER);
                SetWindowPos(progress, HWND_TOP, rect.left + 8, rect.bottom - (24), rect.right - rect.left - (8+64+8+64+8+8), 8, SWP_NOZORDER);
            }
        },

        WM_SIZE => {
            position_close_button(hwnd);
        },

        WM_CLOSE => {
            unsafe {
                DestroyWindow(hwnd);
            }
            return 0;
        }

        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            return 0;
        },

        WM_NCHITTEST => unsafe {
            let mut hit = DefWindowProcW(hwnd, msg, wparam, lparam);
            if hit == HTCLIENT {
                hit = HTCAPTION;
            }
            return hit;
        },

        WM_COMMAND => match wparam as i32 {
            BTN_NEXT => {
                println!("Next ->");
            },

            BTN_BACK => {
                println!("<- Back");
            },

            _ => {}
        },

        _ => {}
    }
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

fn main() -> Result<(), Box<std::error::Error>> {
    unsafe {
        H_INSTANCE = GetModuleHandleW(null_mut());
    }
    register_custom_classes();

    load_brushes();

    let wnd_class = WNDCLASSW {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: unsafe { H_INSTANCE },
        lpszClassName: win32_string("test_window").as_ptr(),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: null_mut(),
        hCursor: unsafe { LoadCursorW(H_INSTANCE, IDC_ARROW) },
        hbrBackground: unsafe { BRUSH_POLAR_0 },
        lpszMenuName: null_mut(),
    };
    unsafe { RegisterClassW(&wnd_class) };

    // centre the window on the desktop
    let desktop = unsafe { GetDesktopWindow() };
    let mut rect = default_rect();
    unsafe { GetClientRect(desktop, &mut rect) };

    let hwnd = unsafe { CreateWindowExW(
            0,
            win32_string("test_window").as_ptr(),
            win32_string("Win32 Test").as_ptr(),
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_POPUP | WS_BORDER,
            (rect.right - 512) / 2,
            (rect.bottom - 256) / 2,
            512, 256,
            null_mut(),
            null_mut(),
            H_INSTANCE,
            null_mut(),
        ) };
    unsafe { ShowWindow(hwnd, SW_SHOW) };

    unsafe {
        let mut msg: MSG = mem::uninitialized();
        while GetMessageW(&mut msg as *mut MSG, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg as *const MSG);
            DispatchMessageW(&msg as *const MSG);
        }
    }

    unload_brushes();
    unregister_custom_classes();
    Ok(())
}
