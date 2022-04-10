use neon::prelude::*;

struct POINT {
    x: libc::c_long,
    y: libc::c_long
}

#[link(name = "user32")]
extern "C" {
    fn GetCursorPos(lpPoint: *mut POINT) -> bool;
    fn GetAsyncKeyState(vKey: libc::c_int) -> libc::c_short;
}

fn GetAsyncKeyState_wrapper(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let vKey: f64 = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let vKey: i32 = vKey as libc::c_int;
    let result: i16 = unsafe { GetAsyncKeyState(vKey) };
    return Ok(cx.boolean(result != 0))
}

fn GetLeftMouseButton(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let mut btn: i16 = 0;
    unsafe {
        btn = GetAsyncKeyState(0x1);
    }
    return Ok(cx.boolean(btn != 0));
}

fn GetMiddleMouseButton(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let mut btn: i16 = 0;
    unsafe {
        btn = GetAsyncKeyState(0x4);
    }
    return Ok(cx.boolean(btn != 0));
}

fn GetRightMouseButton(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let mut btn: i16 = 0;
    unsafe {
        btn = GetAsyncKeyState(0x2);
    }
    return Ok(cx.boolean(btn != 0));
}

fn GetCursorPos_wrapper(mut cx: FunctionContext) -> JsResult<JsArray> {
    let mut output_arr: Handle<JsArray> = cx.empty_array();
    let mut point: POINT = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }
    let px: Handle<JsNumber> = cx.number(point.y as f64);
    let py: Handle<JsNumber> = cx.number(point.x as f64);
    output_arr.set(&mut cx, 0, px);
    output_arr.set(&mut cx, 1, py);
    return  Ok(output_arr);
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("GetCursorPos", GetCursorPos_wrapper);
    cx.export_function("GetAsyncKeyState", GetAsyncKeyState_wrapper);
    cx.export_function("GetRightMouseButton", GetRightMouseButton);
    cx.export_function("GetLeftMouseButton", GetLeftMouseButton);
    cx.export_function("GetMiddleMouseButton", GetMiddleMouseButton);
    return Ok(());
}
