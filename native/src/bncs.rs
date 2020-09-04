#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use std::path::Path;

#[link(name="bncsutil_static", kind="static")]
extern {
    // this is rustified prototype of the function from our C library
    pub fn bncsutil_getVersion() -> u64;
    pub fn bncsutil_getVersionString(outbuf: *const i8) -> ::std::os::raw::c_int;
    pub fn getExeInfo(
        file_name: *const i8,
        exe_info: *mut ::std::os::raw::c_char,
        exe_info_size: usize,
        version: *mut u32,
        platform: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

pub fn version() -> u64 {
    unsafe {
        bncsutil_getVersion()
    }
}

pub fn version_string() -> String {
    unsafe {
        let mut exe_info_vec : Vec<i8> = vec![0i8; 1024];
        let exe_info_slice = exe_info_vec.as_mut_slice();
        let exe_info_ptr = exe_info_slice.as_mut_ptr();
        let length = bncsutil_getVersionString(exe_info_ptr);
        let exe_info_string = String::from_utf8(exe_info_slice.iter().map(|&c| c as u8).collect()).unwrap();
        let exe_info: String = exe_info_string.chars().take(length as usize).collect();

        println!("exe_info {:?}", exe_info);
        println!("length {:?}", length);

        exe_info
    }
}

pub fn get_exe_info(path: &Path) -> (i32, String, u32) {
    unsafe {
        let path_str = path.to_str().unwrap();

        let s = CString::new(path_str).unwrap();
        let ptr = s.as_ptr();
        println!("Cstrinf path {:?} !", s);
        let mut exe_version: u32 = 0;
        let mut exe_info_vec : Vec<i8> = vec![0i8; 1024];
        let exe_info_slice = exe_info_vec.as_mut_slice();
        let exe_info_ptr = exe_info_slice.as_mut_ptr();
        let length = getExeInfo(ptr, exe_info_ptr, 1024 as usize, &mut exe_version, 1 as i32);
        let exe_info_string = String::from_utf8(exe_info_slice.iter().map(|&c| c as u8).collect()).unwrap();
        let exe_info: String = exe_info_string.chars().take(length as usize).collect();

        println!("exe_info {:?}", exe_info);
        println!("exe_version {:?}", exe_version);
        println!("length {:?}", length);

        (
            length,
            exe_info,
            exe_version
        )
    }
}

#[cfg(test)]
mod bncs_tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), 10300);
    }

    #[test]
    fn test_version_string() {
        assert_eq!(version_string(), "1.3.0");
    }

    #[test]
    fn test_get_exe_info() {
        let path = Path::new("../mock/war3.exe");
        let info =  String::from("war3.exe 12/09/16 06:05:09 515048");

        assert_eq!(get_exe_info(path), (33 as i32, info, 18547117 as u32));
    }
}
