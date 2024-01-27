#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate napi_derive;
extern crate libc;

use napi::*;
use std::path::Path;
use std::path::PathBuf;
use std::convert::TryInto;
use std::collections::HashMap;

mod bncs;

//fn version(mut cx: CallContext) -> Result<JsNumber> {
//    unsafe {
//        Ok(cx.number(bncsutil_getVersion() as f64))
//    }
//}
//include!("./bncsutil.rs");

//fn create_client_public_key(mut cx: CallContext) -> Result<JsBuffer> {
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

#[napi]
fn version() -> f64 {
    bncs::version() as f64
}

#[napi]
fn version_string() -> String {
    bncs::version_string()
}

#[napi(object)]
pub struct ExeInfo {
    pub exe_info: String,
    pub exe_version: f64
}

#[napi]
fn get_exe_info(path_string: String) -> ExeInfo {
    // let path_string = cx.get::<JsString>(0)?.try_into()?;
    let path = PathBuf::from(path_string);
    let (_, exe_info, exe_version) = bncs::get_exe_info(&path);

    // let object = cx.env.create_object();
    // let js_info = cx.env.create_string(&exe_info);
    // let js_version = cx.env.create_double(exe_version as f64);

    // object.set("exe_info", js_info).unwrap();
    // object.set("exe_version", js_version).unwrap();

    ExeInfo {
        exe_info,
        exe_version: exe_version.into()
    }
}

// value: String, files: Vec<&Path>, mpqNumber: i32
#[napi]
fn check_revision(value: String, files: Vec<String>, mpq_number: i32) -> u32 {
    // let value_string = cx.get::<JsString>(0)?.try_into()?;
    // let files_arr_handle: Handle<JsArray> = cx.get(1)?.try_into()?;
    // let files_vec: Vec<Handle<JsValue>> = files_arr_handle.to_vec(&mut cx)?;
    // let files = files_vec
    //     .iter()
    //     .map(|val| val.to_string(&mut cx).unwrap())
    //     .map(|val| val.value())
    //     .collect::<Vec<String>>();

    let files_ref = files.iter().map(|val| Path::new(val.as_str())).collect();

    // let mpq_number = cx.get::<JsNumber>(2)?.try_into()?;

    bncs::check_revision(value, files_ref, mpq_number as i32)
}

// value: String, file1: &Path, file2: &Path, file3: &Path, mpq_number: i32
#[napi]
fn check_revision_flat(value: String, file1: String, file2: String, file3: String, mpq_number: i32) -> u32 {
    // let value_string = cx.get::<JsString>(0)?.try_into()?;
    // let file1= cx.get::<JsString>(1)?.try_into()?;
    // let file2= cx.get::<JsString>(2)?.try_into()?;
    // let file3= cx.get::<JsString>(3)?.try_into()?;
    // let mpq_number = cx.get::<JsNumber>(4)?.try_into()?;

    let files = vec![file1, file2, file3];
    let files_ref = files.iter().map(|val| Path::new(val.as_str())).collect();

    bncs::check_revision(value, files_ref, mpq_number as i32)
}

// #[module_exports]
// fn init(mut m: JsObject) -> Result<()> {
//   m.create_named_method("version_string", version_string_js);
//   m.create_named_method("version", version_js);
//   m.create_named_method("get_exe_info", get_exe_info_js);
// //   m.create_named_method("check_revision", check_revision_js);
// //   m.create_named_method("check_revision_flat", check_revision_flat_js);

//   Ok(())
// }