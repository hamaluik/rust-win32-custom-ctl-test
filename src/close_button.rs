use std::mem;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use std::alloc::{self, Layout};

use crate::util::default_rect;
use crate::brushes::*;

enum ButtonState {
    Idle,
    Hover,
    Active,
}

static mut HWND_CLOSE_BUTTON: HWND = null_mut();

fn paint_button(hwnd: HWND, state: &ButtonState) {
    let mut ps: PAINTSTRUCT = PAINTSTRUCT {
        hdc: null_mut(),
        fErase: FALSE,
        rcPaint: default_rect(),
        fRestore: FALSE,
        fIncUpdate: FALSE,
        rgbReserved: [0u8; 32],
    };
    let mut rect = default_rect();

    unsafe {
        let hdc = BeginPaint(hwnd, &mut ps);
        GetClientRect(hwnd, &mut rect);

        FillRect(hdc, &rect, match state {
            ButtonState::Idle => BRUSH_POLAR_0,
            ButtonState::Hover => BRUSH_POLAR_1,
            ButtonState::Active => BRUSH_AURORA_0,
        });

        let old_pen = SelectObject(hdc, PEN_SNOW_0 as HGDIOBJ);
        MoveToEx(hdc, rect.left + 8, rect.top + 8, null_mut());
        LineTo(hdc, rect.right - 7, rect.bottom - 7);
        MoveToEx(hdc, rect.right - 8, rect.top + 8, null_mut());
        LineTo(hdc, rect.left + 7, rect.bottom - 7);
        SelectObject(hdc, old_pen);

        EndPaint(hwnd, &mut ps);
    }
}

extern "system" fn close_button_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, 0) as *mut u8 };
    let button_state = ptr as *mut ButtonState;

    match msg {
        WM_NCCREATE => {
            let layout = Layout::new::<ButtonState>();
            unsafe {
                let new_ptr = alloc::alloc(layout);
                if new_ptr as isize == 0 {
                    return FALSE as isize;
                }
                *(new_ptr as *mut ButtonState) = ButtonState::Idle;
                if SetWindowLongPtrW(hwnd, 0, new_ptr as isize) != 0 {
                    return FALSE as isize;
                }
            }
            return TRUE as isize;
        },

        WM_NCDESTROY => {
            if ptr as isize != 0 {
                let layout = Layout::new::<ButtonState>();
                unsafe { alloc::dealloc(ptr, layout) };
            }
            return 0;
        },

        WM_PAINT => {
            paint_button(hwnd, unsafe { &*button_state });
            return 0;
        }

        WM_MOUSEMOVE => unsafe {
            let pos = POINT { x: GET_X_LPARAM(lparam), y: GET_Y_LPARAM(lparam) };
            let mut rect = default_rect();
            GetClientRect(hwnd, &mut rect);

            let mut track = TRACKMOUSEEVENT {
                cbSize: mem::size_of::<TRACKMOUSEEVENT>() as u32,
                dwFlags: TME_LEAVE,
                hwndTrack: hwnd,
                dwHoverTime: HOVER_DEFAULT,
            };
            TrackMouseEvent(&mut track);

            *button_state = if PtInRect(&rect, pos) != 0 {
                match *button_state {
                    ButtonState::Idle => ButtonState::Hover,
                    ButtonState::Hover => ButtonState::Hover,
                    ButtonState::Active => ButtonState::Active,
                }
            }
            else {
                ButtonState::Idle
            };
            InvalidateRect(hwnd, &rect, FALSE);
            return 0;
        },

        WM_MOUSELEAVE => unsafe {
            *button_state = ButtonState::Idle;
            InvalidateRect(hwnd, null_mut(), FALSE);
            return 0;
        },

        WM_LBUTTONDOWN => unsafe {
            *button_state = ButtonState::Active;
            InvalidateRect(hwnd, null_mut(), FALSE);
            return 0;
        },

        WM_LBUTTONUP => unsafe {
            let pos = POINT { x: GET_X_LPARAM(lparam), y: GET_Y_LPARAM(lparam) };
            let mut rect = default_rect();
            GetClientRect(hwnd, &mut rect);

            if let ButtonState::Active = *button_state {
                let parent = GetParent(hwnd);
                PostMessageW(parent, WM_CLOSE, 0, 0);
            }

            *button_state = if PtInRect(&rect, pos) != 0 { ButtonState::Hover } else { ButtonState::Idle };

            InvalidateRect(hwnd, null_mut(), FALSE);
            return 0;
        },

        _ => {}
    }

    return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
}

pub fn register_close_button() {
    let btn_class = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(close_button_proc),
        cbClsExtra: 0,
        cbWndExtra: mem::size_of::<*const i32>() as i32,
        hInstance: unsafe { crate::H_INSTANCE },
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
        lpszClassName: crate::util::win32_string("close_button").as_ptr(),
        hIconSm: null_mut(),
    };
    unsafe { RegisterClassExW(&btn_class) };
}

pub fn unregister_close_button() {
    unsafe { UnregisterClassW(crate::util::win32_string("close_button").as_ptr(), crate::H_INSTANCE) };
}

pub fn create_close_button(parent: HWND) {
    unsafe {
        HWND_CLOSE_BUTTON = CreateWindowExW(
            0,
            crate::util::win32_string("close_button").as_ptr(),
            null_mut(),
            WS_CHILD | WS_VISIBLE,
            0, 0, 0, 0,
            parent,
            null_mut(),
            crate::H_INSTANCE,
            null_mut(),
        );
    }
}

pub fn position_close_button(parent: HWND) {
    let mut rect = default_rect();
    unsafe {
        GetClientRect(parent, &mut rect);
        SetWindowPos(HWND_CLOSE_BUTTON, HWND_TOP, rect.right - 24, rect.top, 24, 24, SWP_NOZORDER);
    }
}
