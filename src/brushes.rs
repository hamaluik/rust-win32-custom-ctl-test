use winapi::shared::windef::HGDIOBJ;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::PS_SOLID;
//use winapi::um::wingdi::RGB;
use winapi::um::wingdi::CreatePen;
use winapi::um::wingdi::CreateSolidBrush;
use std::ptr::null_mut;
use winapi::shared::windef::{HBRUSH, HPEN, COLORREF};

pub static COLOUR_POLAR_0: COLORREF = 0x40342e;
pub static COLOUR_POLAR_1: COLORREF = 0x52423b;
pub static COLOUR_SNOW_0: COLORREF = 0xe9ded8;
pub static COLOUR_AURORA_0: COLORREF = 0x6a61bf;

pub static mut BRUSH_POLAR_0: HBRUSH = null_mut();
pub static mut BRUSH_POLAR_1: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_0: HBRUSH = null_mut();

pub static mut PEN_SNOW_0: HPEN = null_mut();

pub fn load_brushes() {
    unsafe {
        //BRUSH_POLAR_0 = CreateSolidBrush(RGB(46, 52, 64));
        //BRUSH_POLAR_1 = CreateSolidBrush(RGB(59, 66, 82));
        //BRUSH_AURORA_0 = CreateSolidBrush(RGB(191, 97, 106));
        //PEN_SNOW_0 = CreatePen(PS_SOLID as i32, 2, RGB(216, 222, 233));
        BRUSH_POLAR_0 = CreateSolidBrush(COLOUR_POLAR_0);
        BRUSH_POLAR_1 = CreateSolidBrush(COLOUR_POLAR_1);
        BRUSH_AURORA_0 = CreateSolidBrush(COLOUR_AURORA_0);
        PEN_SNOW_0 = CreatePen(PS_SOLID as i32, 1, COLOUR_SNOW_0);
    }
}

pub fn unload_brushes() {
    unsafe {
        DeleteObject(BRUSH_POLAR_0 as HGDIOBJ);
        DeleteObject(BRUSH_POLAR_1 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_0 as HGDIOBJ);
        DeleteObject(PEN_SNOW_0 as HGDIOBJ);
    }
}
