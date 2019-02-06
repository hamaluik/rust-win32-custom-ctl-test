use winapi::shared::windef::HGDIOBJ;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::PS_SOLID;
use winapi::um::wingdi::CreatePen;
use winapi::um::wingdi::CreateSolidBrush;
use std::ptr::null_mut;
use winapi::shared::windef::{HBRUSH, HPEN, COLORREF};

#[allow(dead_code)]
pub enum PolarNight {
    P0, P1, P2, P3
}

#[allow(dead_code)]
pub enum SnowStorm {
    S0, S1, S2
}

#[allow(dead_code)]
pub enum Frost {
    F0, F1, F2, F3
}

#[allow(dead_code)]
pub enum Aurora {
    Red, Orange, Yellow, Green, Purple
}

pub static COLOUR_POLAR_0: COLORREF = 0x40342e;
pub static COLOUR_POLAR_1: COLORREF = 0x52423b;
pub static COLOUR_POLAR_2: COLORREF = 0x5e4c43;
pub static COLOUR_POLAR_3: COLORREF = 0x6a564c;
pub static COLOUR_SNOW_0: COLORREF = 0xe9ded8;
pub static COLOUR_SNOW_1: COLORREF = 0xf0e9e5;
pub static COLOUR_SNOW_2: COLORREF = 0xf4efec;
pub static COLOUR_FROST_0: COLORREF = 0xbbbc8f;
pub static COLOUR_FROST_1: COLORREF = 0xd0c088;
pub static COLOUR_FROST_2: COLORREF = 0xc1a181;
pub static COLOUR_FROST_3: COLORREF = 0xac815e;
pub static COLOUR_AURORA_0: COLORREF = 0x6a61bf;
pub static COLOUR_AURORA_1: COLORREF = 0x7087d0;
pub static COLOUR_AURORA_2: COLORREF = 0x8bcbeb;
pub static COLOUR_AURORA_3: COLORREF = 0x8cbea3;
pub static COLOUR_AURORA_4: COLORREF = 0xad8eb4;

pub static mut BRUSH_POLAR_0: HBRUSH = null_mut();
pub static mut BRUSH_POLAR_1: HBRUSH = null_mut();
pub static mut BRUSH_POLAR_2: HBRUSH = null_mut();
pub static mut BRUSH_POLAR_3: HBRUSH = null_mut();
pub static mut BRUSH_SNOW_0: HBRUSH = null_mut();
pub static mut BRUSH_SNOW_1: HBRUSH = null_mut();
pub static mut BRUSH_SNOW_2: HBRUSH = null_mut();
pub static mut BRUSH_FROST_0: HBRUSH = null_mut();
pub static mut BRUSH_FROST_1: HBRUSH = null_mut();
pub static mut BRUSH_FROST_2: HBRUSH = null_mut();
pub static mut BRUSH_FROST_3: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_0: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_1: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_2: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_3: HBRUSH = null_mut();
pub static mut BRUSH_AURORA_4: HBRUSH = null_mut();

pub static mut PEN_POLAR_0: HPEN = null_mut();
pub static mut PEN_POLAR_1: HPEN = null_mut();
pub static mut PEN_POLAR_2: HPEN = null_mut();
pub static mut PEN_POLAR_3: HPEN = null_mut();
pub static mut PEN_SNOW_0: HPEN = null_mut();
pub static mut PEN_SNOW_1: HPEN = null_mut();
pub static mut PEN_SNOW_2: HPEN = null_mut();
pub static mut PEN_FROST_0: HPEN = null_mut();
pub static mut PEN_FROST_1: HPEN = null_mut();
pub static mut PEN_FROST_2: HPEN = null_mut();
pub static mut PEN_FROST_3: HPEN = null_mut();
pub static mut PEN_AURORA_0: HPEN = null_mut();
pub static mut PEN_AURORA_1: HPEN = null_mut();
pub static mut PEN_AURORA_2: HPEN = null_mut();
pub static mut PEN_AURORA_3: HPEN = null_mut();
pub static mut PEN_AURORA_4: HPEN = null_mut();

pub fn load_brushes() {
    unsafe {
        BRUSH_POLAR_0 = CreateSolidBrush(COLOUR_POLAR_0);
        BRUSH_POLAR_1 = CreateSolidBrush(COLOUR_POLAR_1);
        BRUSH_POLAR_2 = CreateSolidBrush(COLOUR_POLAR_2);
        BRUSH_POLAR_3 = CreateSolidBrush(COLOUR_POLAR_3);
        BRUSH_SNOW_0 = CreateSolidBrush(COLOUR_SNOW_0);
        BRUSH_SNOW_1 = CreateSolidBrush(COLOUR_SNOW_1);
        BRUSH_SNOW_2 = CreateSolidBrush(COLOUR_SNOW_2);
        BRUSH_FROST_0 = CreateSolidBrush(COLOUR_FROST_0);
        BRUSH_FROST_1 = CreateSolidBrush(COLOUR_FROST_1);
        BRUSH_FROST_2 = CreateSolidBrush(COLOUR_FROST_2);
        BRUSH_FROST_3 = CreateSolidBrush(COLOUR_FROST_3);
        BRUSH_AURORA_0 = CreateSolidBrush(COLOUR_AURORA_0);
        BRUSH_AURORA_1 = CreateSolidBrush(COLOUR_AURORA_1);
        BRUSH_AURORA_2 = CreateSolidBrush(COLOUR_AURORA_2);
        BRUSH_AURORA_3 = CreateSolidBrush(COLOUR_AURORA_3);
        BRUSH_AURORA_4 = CreateSolidBrush(COLOUR_AURORA_4);

        PEN_POLAR_0 = CreatePen(PS_SOLID as i32, 1, COLOUR_POLAR_0);
        PEN_POLAR_1 = CreatePen(PS_SOLID as i32, 1, COLOUR_POLAR_1);
        PEN_POLAR_2 = CreatePen(PS_SOLID as i32, 1, COLOUR_POLAR_2);
        PEN_POLAR_3 = CreatePen(PS_SOLID as i32, 1, COLOUR_POLAR_3);
        PEN_SNOW_0 = CreatePen(PS_SOLID as i32, 1, COLOUR_SNOW_0);
        PEN_SNOW_1 = CreatePen(PS_SOLID as i32, 1, COLOUR_SNOW_1);
        PEN_SNOW_2 = CreatePen(PS_SOLID as i32, 1, COLOUR_SNOW_2);
        PEN_FROST_0 = CreatePen(PS_SOLID as i32, 1, COLOUR_FROST_0);
        PEN_FROST_1 = CreatePen(PS_SOLID as i32, 1, COLOUR_FROST_1);
        PEN_FROST_2 = CreatePen(PS_SOLID as i32, 1, COLOUR_FROST_2);
        PEN_FROST_3 = CreatePen(PS_SOLID as i32, 1, COLOUR_FROST_3);
        PEN_AURORA_0 = CreatePen(PS_SOLID as i32, 1, COLOUR_AURORA_0);
        PEN_AURORA_1 = CreatePen(PS_SOLID as i32, 1, COLOUR_AURORA_1);
        PEN_AURORA_2 = CreatePen(PS_SOLID as i32, 1, COLOUR_AURORA_2);
        PEN_AURORA_3 = CreatePen(PS_SOLID as i32, 1, COLOUR_AURORA_3);
        PEN_AURORA_4 = CreatePen(PS_SOLID as i32, 1, COLOUR_AURORA_4);
    }
}

pub fn unload_brushes() {
    unsafe {
        DeleteObject(BRUSH_POLAR_0 as HGDIOBJ);
        DeleteObject(BRUSH_POLAR_1 as HGDIOBJ);
        DeleteObject(BRUSH_POLAR_2 as HGDIOBJ);
        DeleteObject(BRUSH_POLAR_3 as HGDIOBJ);
        DeleteObject(BRUSH_SNOW_0 as HGDIOBJ);
        DeleteObject(BRUSH_SNOW_1 as HGDIOBJ);
        DeleteObject(BRUSH_SNOW_2 as HGDIOBJ);
        DeleteObject(BRUSH_FROST_0 as HGDIOBJ);
        DeleteObject(BRUSH_FROST_1 as HGDIOBJ);
        DeleteObject(BRUSH_FROST_2 as HGDIOBJ);
        DeleteObject(BRUSH_FROST_3 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_0 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_1 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_2 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_3 as HGDIOBJ);
        DeleteObject(BRUSH_AURORA_4 as HGDIOBJ);
        DeleteObject(PEN_POLAR_0 as HGDIOBJ);
        DeleteObject(PEN_POLAR_1 as HGDIOBJ);
        DeleteObject(PEN_POLAR_2 as HGDIOBJ);
        DeleteObject(PEN_POLAR_3 as HGDIOBJ);
        DeleteObject(PEN_SNOW_0 as HGDIOBJ);
        DeleteObject(PEN_SNOW_1 as HGDIOBJ);
        DeleteObject(PEN_SNOW_2 as HGDIOBJ);
        DeleteObject(PEN_FROST_0 as HGDIOBJ);
        DeleteObject(PEN_FROST_1 as HGDIOBJ);
        DeleteObject(PEN_FROST_2 as HGDIOBJ);
        DeleteObject(PEN_FROST_3 as HGDIOBJ);
        DeleteObject(PEN_AURORA_0 as HGDIOBJ);
        DeleteObject(PEN_AURORA_1 as HGDIOBJ);
        DeleteObject(PEN_AURORA_2 as HGDIOBJ);
        DeleteObject(PEN_AURORA_3 as HGDIOBJ);
        DeleteObject(PEN_AURORA_4 as HGDIOBJ);
    }
}
