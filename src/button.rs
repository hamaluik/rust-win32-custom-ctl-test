use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winnt::PVOID;
use std::mem;
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use std::ptr::null_mut;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use std::alloc::{self, Layout};

use crate::util::{default_rect, win32_string};
use crate::brushes::*;

enum ButtonState {
    Idle,
    Hover,
    Active,
}

struct ButtonData {
    state: ButtonState,
    font: HFONT,
    custom_font: bool,
}

static BUTTON_CLASS_NAME: &str = "_custom_button_";
static mut LPFN_BUTTON_PROC: WNDPROC = None;
static mut CB_WND_EXTRA: i32 = 0;

fn custom_paint(hwnd: HWND, hdc: HDC, rect: &mut RECT, _erase: BOOL, data: &ButtonData) {
    unsafe {
        FillRect(hdc, rect, match data.state {
            ButtonState::Idle => BRUSH_POLAR_0,
            ButtonState::Hover => BRUSH_POLAR_1,
            ButtonState::Active => BRUSH_AURORA_0,
        });

        let old_pen = SelectObject(hdc, PEN_SNOW_0 as HGDIOBJ);
        MoveToEx(hdc, rect.left, rect.top, null_mut());
        LineTo(hdc, rect.right, rect.top);
        LineTo(hdc, rect.right, rect.bottom);
        LineTo(hdc, rect.left, rect.bottom);
        LineTo(hdc, rect.left, rect.top);
        SelectObject(hdc, old_pen);

        SetBkMode(hdc, TRANSPARENT as i32);

        let len = GetWindowTextLengthW(hwnd) + 1;
        let mut text: Vec<u16> = vec![0u16; len as usize];
        GetWindowTextW(hwnd, text.as_mut_ptr(), len);
        
        let old_font = SelectObject(hdc, data.font as HGDIOBJ);
        SetTextColor(hdc, crate::brushes::COLOUR_SNOW_0);
        DrawTextW(hdc, text.as_ptr(), -1, rect, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
        SelectObject(hdc, old_font);
    }
}

extern "system" fn button_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let ptr = unsafe { GetWindowLongPtrW(hwnd, CB_WND_EXTRA) as *mut u8 };
    let button_data = ptr as *mut ButtonData;

    match msg {
        WM_NCCREATE => {
            unsafe {
                if CallWindowProcW(LPFN_BUTTON_PROC, hwnd, msg, wparam, lparam) == 0 {
                    return FALSE as isize;
                }
            }

            let layout = Layout::new::<ButtonData>();
            unsafe {
                let new_ptr = alloc::alloc(layout);
                if new_ptr as isize == 0 {
                    return FALSE as isize;
                }
                
                let mut metrics: NONCLIENTMETRICSW = NONCLIENTMETRICSW {
                    cbSize: mem::size_of::<NONCLIENTMETRICSW>() as u32,
                    iBorderWidth: 0,
                    iScrollWidth: 0,
                    iScrollHeight: 0,
                    iCaptionWidth: 0,
                    iCaptionHeight: 0,
                    lfCaptionFont: mem::uninitialized(),
                    iSmCaptionWidth: 0,
                    iSmCaptionHeight: 0,
                    lfSmCaptionFont: mem::uninitialized(),
                    iMenuWidth: 0,
                    iMenuHeight: 0,
                    lfMenuFont: mem::uninitialized(),
                    lfStatusFont: mem::uninitialized(),
                    lfMessageFont: mem::uninitialized(),
                    iPaddedBorderWidth: 0,
                };
                let font = if SystemParametersInfoW(SPI_GETNONCLIENTMETRICS, mem::size_of::<NONCLIENTMETRICSW>() as u32, &mut metrics as *mut _ as PVOID, 0) == 0 {
                    eprintln!("failed to get SPI_GETNONCLIENTMETRICS: {}", GetLastError());
                    GetStockObject(SYSTEM_FONT as i32) as HFONT
                }
                else {
                    CreateFontIndirectW(&metrics.lfMessageFont)
                };

                *(new_ptr as *mut ButtonData) = ButtonData {
                    state: ButtonState::Idle,
                    font,
                    custom_font: false
                };
                if SetWindowLongPtrW(hwnd, CB_WND_EXTRA, new_ptr as isize) != 0 {
                    return FALSE as isize;
                }
            }
            return TRUE as isize;
        },

        WM_NCDESTROY => {
            if ptr as isize != 0 {
                let layout = Layout::new::<ButtonData>();
                unsafe {
                    if !(*button_data).custom_font {
                        DeleteObject((*button_data).font as HGDIOBJ);
                    }
                    
                    alloc::dealloc(ptr, layout)
                };
            }
            return 0;
        },

        WM_SETFONT => {
            unsafe {
                if !(*button_data).custom_font {
                    DeleteObject((*button_data).font as HGDIOBJ);
                }
                (*button_data).font = wparam as HFONT;
                (*button_data).custom_font = true;
                if lparam != 0 {
                    InvalidateRect(hwnd, null_mut(), TRUE);
                }
            }
            return 0;
        },

        WM_GETFONT => {
            unsafe {
                return (*button_data).font as LRESULT;
            }
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
                custom_paint(hwnd, ps.hdc, &mut ps.rcPaint, ps.fErase, &*button_data);
                EndPaint(hwnd, &ps);
            }
            return 0;
        },

        WM_PRINTCLIENT => {
            let mut rc: RECT = default_rect();
            unsafe {
                GetClientRect(hwnd, &mut rc);
                custom_paint(hwnd, wparam as HDC, &mut rc, TRUE, &*button_data);
            }
            return 0;
        },

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

            (*button_data).state = if PtInRect(&rect, pos) != 0 {
                match (*button_data).state {
                    ButtonState::Idle => ButtonState::Hover,
                    ButtonState::Hover => ButtonState::Hover,
                    ButtonState::Active => ButtonState::Active,
                }
            }
            else {
                ButtonState::Idle
            };
            InvalidateRect(hwnd, &rect, FALSE);
            //return 0;
        },

        WM_MOUSELEAVE => unsafe {
            (*button_data).state = ButtonState::Idle;
            InvalidateRect(hwnd, null_mut(), FALSE);
            //return 0;
        },

        WM_LBUTTONDOWN => unsafe {
            (*button_data).state = ButtonState::Active;
            InvalidateRect(hwnd, null_mut(), FALSE);
            //return 0;
        },

        WM_LBUTTONUP => unsafe {
            let pos = POINT { x: GET_X_LPARAM(lparam), y: GET_Y_LPARAM(lparam) };
            let mut rect = default_rect();
            GetClientRect(hwnd, &mut rect);

            //if let ButtonState::Active = *button_state {
            //    let parent = GetParent(hwnd);
            //    PostMessageW(parent, WM_CLOSE, 0, 0);
            //}

            (*button_data).state = if PtInRect(&rect, pos) != 0 { ButtonState::Hover } else { ButtonState::Idle };

            InvalidateRect(hwnd, null_mut(), FALSE);
            //return 0;
        },

        _ => {}
    }

    return unsafe { CallWindowProcW(LPFN_BUTTON_PROC, hwnd, msg, wparam, lparam) };
}

pub fn register_button() {
    let mut btn_class = WNDCLASSEXW {
        cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(button_proc),
        cbClsExtra: 0,
        cbWndExtra: mem::size_of::<*mut ButtonData>() as i32,
        hInstance: unsafe { crate::H_INSTANCE },
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
        lpszClassName: null_mut(),
        hIconSm: null_mut(),
    };
    unsafe {
        GetClassInfoExW(crate::H_INSTANCE, win32_string("BUTTON").as_ptr(), &mut btn_class);
        CB_WND_EXTRA = btn_class.cbWndExtra;
        LPFN_BUTTON_PROC = btn_class.lpfnWndProc;

        btn_class.lpszClassName = win32_string(BUTTON_CLASS_NAME).as_ptr();
        btn_class.style &= !CS_GLOBALCLASS;
        btn_class.hInstance = crate::H_INSTANCE;
        btn_class.cbWndExtra += mem::size_of::<*mut ButtonData>() as i32;
        btn_class.lpfnWndProc = Some(button_proc);

        RegisterClassExW(&btn_class)
    };
}

pub fn unregister_button() {
    unsafe { UnregisterClassW(win32_string(BUTTON_CLASS_NAME).as_ptr(), crate::H_INSTANCE) };
}

pub fn create_button(parent: HWND, text: &str) -> HWND {
    unsafe {
        CreateWindowExW(
            0,
            win32_string(BUTTON_CLASS_NAME).as_ptr(),
            win32_string(text).as_ptr(),
            WS_CHILD | WS_VISIBLE,
            0, 0, 0, 0,
            parent,
            null_mut(),
            crate::H_INSTANCE,
            null_mut(),
        )
    }
}
