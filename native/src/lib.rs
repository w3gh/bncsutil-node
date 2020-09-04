#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//#[macro_use]
extern crate neon;
extern crate libc;

use bncs::*;
use std::path::Path;
use std::path::PathBuf;
use neon::prelude::*;

mod bncs;

//fn version(mut cx: FunctionContext) -> JsResult<JsNumber> {
//    unsafe {
//        Ok(cx.number(bncsutil_getVersion() as f64))
//    }
//}
//include!("./bncsutil.rs");

//fn create_client_public_key(mut cx: FunctionContext) -> JsResult<JsBuffer> {
//    let mut username = cx.argument::<JsString>(0)?.value();
//    let mut password = cx.argument::<JsString>(1)?.value();
//    let mut result: Vec<i8> = Vec::new();
//    let s = result.as_mut_ptr();
//
//    let c_u_b = CString::new(username).expect("username to CString");
//    let c_p_b = CString::new(password).expect("password to CString");
//    let mut c_u = CStr::from_bytes_with_nul(c_u_b.to_bytes_with_nul()).expect("try convert username to cstr");
//    let mut c_p = CStr::from_bytes_with_nul(c_p_b.to_bytes_with_nul()).expect("try convert password to cstr");
//
//    unsafe {
//        nls_get_A(nls_init(c_u.as_ptr(), c_p.as_ptr()), s);
//    }
//
//
//    let buffer:Handle<JsBuffer> = cx.buffer(result.len() as u32)?;
//
//    result.iter().enumerate().for_each(|e| {
//        let (i, obj) = e;
//        let js_number = cx.number(result[i]);
//        let _ = buffer.set(&mut cx, i as u32, js_number);
//    });

//    Ok(buffer)

//}

fn version_js(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(version() as f64))
}

fn version_string_js(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(version_string()))
}

fn get_exe_info_js(mut cx: FunctionContext) -> JsResult<JsObject> {
    let path_string = cx.argument::<JsString>(0)?.value();
    let path = PathBuf::from(path_string);
    let (length,
        exe_info,
        exe_version) = get_exe_info(&path);

    let object = JsObject::new(&mut cx);
    let js_info = cx.string(&exe_info);
    let js_version = cx.number(exe_version as f64);
    object.set(&mut cx, "exe_info", js_info).unwrap();
    object.set(&mut cx, "exe_version", js_version).unwrap();

    Ok(object)
}

register_module!(mut m, {
    m.export_function("version_string", version_string_js);
    m.export_function("version", version_js);
    m.export_function("get_exe_info", get_exe_info_js);

    Ok(())
});
