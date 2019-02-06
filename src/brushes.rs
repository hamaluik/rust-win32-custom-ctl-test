use winapi::shared::windef::HGDIOBJ;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::PS_SOLID;
use winapi::um::wingdi::RGB;
use winapi::um::wingdi::CreatePen;
use winapi::um::wingdi::CreateSolidBrush;
use std::ptr::null_mut;
use winapi::shared::windef::{HBRUSH, HPEN};

pub static mut BRUSH_POLAR_0: HBRUSH = null_mut();
pub static mut BRUSH_POLAR_1: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_0: HBRUSH = null_mut();

pub static mut PEN_SNOW_0: HPEN = null_mut();

pub fn load_brushes() {
    unsafe {
        BRUSH_POLAR_0 = CreateSolidBrush(RGB(46, 52, 64));
        BRUSH_POLAR_1 = CreateSolidBrush(RGB(59, 66, 82));
        BRUSH_AURORA_0 = CreateSolidBrush(RGB(191, 97, 106));
        PEN_SNOW_0 = CreatePen(PS_SOLID as i32, 2, RGB(216, 222, 233));
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
