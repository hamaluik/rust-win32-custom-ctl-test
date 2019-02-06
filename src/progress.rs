use winapi::um::errhandlingapi::GetLastError;
use std::mem;
use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::alloc::{self, Layout};

use crate::util::{default_rect, win32_string};
use crate::brushes::*;

struct ProgressData {
    progress: f64,
}

static PROGRESS_CLASS_NAME: &str = "_custom_progress_";

fn custom_paint(_: HWND, hdc: HDC, rect: &mut RECT, _erase: BOOL, data: &ProgressData) {
    unsafe {
        let progress = data.progress.max(0.0).min(1.0);
        let fg: RECT = RECT {
            right: (progress * ((rect.right - rect.left) as f64)) as i32 + rect.left,
            ..*rect
        };
        let bg: RECT = RECT {
            left: (progress * ((rect.right - rect.left) as f64)) as i32 + rect.left,
            ..*rect
        };
        FillRect(hdc, &bg, BRUSH_POLAR_1);
        FillRect(hdc, &fg, BRUSH_SNOW_2);
    }
}

extern "system" fn progress_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, 0) as *mut u8 };
    let progress_data = ptr as *mut ProgressData;

    match msg {
        WM_NCCREATE => {
            let layout = Layout::new::<ProgressData>();
            unsafe {
                let new_ptr = alloc::alloc(layout);
                if new_ptr as isize == 0 {
                    return FALSE as isize;
                }

                *(new_ptr as *mut ProgressData) = ProgressData {
                    progress: 0.0
                };
                if SetWindowLongPtrW(hwnd, 0, new_ptr as isize) != 0 {
                    return FALSE as isize;
                }
            }
            return TRUE as isize;
        },

        WM_NCDESTROY => {
            if ptr as isize != 0 {
                let layout = Layout::new::<ProgressData>();
                unsafe {
                    alloc::dealloc(ptr, layout)
                };
            }
            return 0;
        },

        WM_ERASEBKGND => {
            return 0;
        },

        WM_PAINT => {
            let mut ps: PAINTSTRUCT = PAINTSTRUCT {
                hdc: null_mut(),
                fErase: FALSE,
                rcPaint: default_rect(),
                fRestore: FALSE,
                fIncUpdate: FALSE,
                rgbReserved: [0u8; 32],
            };
            unsafe {
                BeginPaint(hwnd, &mut ps);
                custom_paint(hwnd, ps.hdc, &mut ps.rcPaint, ps.fErase, &*progress_data);
                EndPaint(hwnd, &ps);
            }
            return 0;
        },

        WM_PRINTCLIENT => {
            let mut rc: RECT = default_rect();
            unsafe {
                GetClientRect(hwnd, &mut rc);
                custom_paint(hwnd, wparam as HDC, &mut rc, TRUE, &*progress_data);
            }
            return 0;
        },

        _ => {}
    }

    return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
}

pub fn register_progress() {
    let progress_class = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(progress_proc),
        cbClsExtra: 0,
        cbWndExtra: mem::size_of::<*mut ProgressData>() as i32,
        hInstance: unsafe { crate::H_INSTANCE },
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
        lpszClassName: win32_string(PROGRESS_CLASS_NAME).as_ptr(),
        hIconSm: null_mut(),
    };
    unsafe {
        RegisterClassExW(&progress_class)
    };
}

pub fn unregister_progress() {
    unsafe { UnregisterClassW(win32_string(PROGRESS_CLASS_NAME).as_ptr(), crate::H_INSTANCE) };
}

pub fn set_progress(bar: HWND, progress: f64) {
    unsafe {
        let ptr = GetWindowLongPtrW(bar, 0) as *mut u8;
        let progress_data = ptr as *mut ProgressData;
        (*progress_data).progress = progress;
        if SetWindowLongPtrW(bar, 0, ptr as isize) == 0 {
            eprintln!("Failed to set progress of progressbar: {}", GetLastError());
        }
    }
}

pub fn create_progress(parent: HWND, progress: f64) -> HWND {
    unsafe {
        let handle = CreateWindowExW(
            0,
            win32_string(PROGRESS_CLASS_NAME).as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE,
            0, 0, 0, 0,
            parent,
            null_mut(),
            crate::H_INSTANCE,
            null_mut(),
        );

        set_progress(handle, progress);

        handle
    }
}
