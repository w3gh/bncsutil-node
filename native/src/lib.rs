#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]



//#[macro_use]
extern crate neon;
extern crate libc;

//use std::ffi::{CStr, CString};
use neon::prelude::*;

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



#[link(name="bncsutil_static", kind="static")]
extern {
    // this is rustified prototype of the function from our C library
    pub fn bncsutil_getVersion() -> u64;
//    pub fn bncsutil_getVersionString(outbuf: *mut ::std::os::raw::c_char) -> u64;
}

pub fn version() -> u64 {
    unsafe {
        bncsutil_getVersion()
    }
}

fn version_js(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(version() as f64))
}

register_module!(mut m, {
    m.export_function("version", version_js)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), 10300);
    }
}
