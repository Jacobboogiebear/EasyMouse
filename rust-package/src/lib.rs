use neon::prelude::*;

struct POINT {
    x: libc::c_long,
    y: libc::c_long
}

#[link(name = "user32")]
extern "C" {
    fn GetCursorPos(lpPoint: *mut POINT) -> bool;
}

fn GetCursorPos_wrapper(mut cx: FunctionContext) -> JsResult<JsArray> {
    let mut output_arr: Handle<JsArray> = cx.empty_array();
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }
    let px = cx.number(point.y as f64);
    let py = cx.number(point.x as f64);
    output_arr.set(&mut cx, 0, px);
    output_arr.set(&mut cx, 1, py);
    return  Ok(output_arr);
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("GetCursorPos", GetCursorPos_wrapper);
    return Ok(());
}
