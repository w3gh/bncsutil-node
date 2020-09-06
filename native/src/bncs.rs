#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::ffi::CString;
use std::path::Path;
use std::os::raw::c_char;

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
    pub fn checkRevisionFlat(
        valueString: *const ::std::os::raw::c_char,
        file1: *const ::std::os::raw::c_char,
        file2: *const ::std::os::raw::c_char,
        file3: *const ::std::os::raw::c_char,
        mpqNumber: ::std::os::raw::c_int,
        checksum: *mut ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn checkRevision(
        valueString: *const ::std::os::raw::c_char,
        files: *mut *const ::std::os::raw::c_char,
        numFiles: ::std::os::raw::c_int,
        mpqNumber: ::std::os::raw::c_int,
        checksum: *mut ::std::os::raw::c_ulong,
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

        exe_info
    }
}

pub fn get_exe_info(path: &Path) -> (i32, String, u32) {
    unsafe {
        let path_str = path.to_str().unwrap();

        let s = CString::new(path_str).unwrap();
        let ptr = s.as_ptr();
        let mut exe_version: u32 = 0;
        let mut exe_info_vec : Vec<i8> = vec![0i8; 1024];
        let exe_info_slice = exe_info_vec.as_mut_slice();
        let exe_info_ptr = exe_info_slice.as_mut_ptr();
        let length = getExeInfo(ptr, exe_info_ptr, 1024 as usize, &mut exe_version, 1 as i32);
        let exe_info_string = String::from_utf8(exe_info_slice.iter().map(|&c| c as u8).collect()).unwrap();
        let exe_info: String = exe_info_string.chars().take(length as usize).collect();

        (
            length,
            exe_info,
            exe_version
        )
    }
}

pub fn check_revision(value: String, files: Vec<&Path>, mpq_number: i32) -> u32 {
    unsafe {
        let files_str = files.iter()
            .map(|val| CString::new(val.to_str().unwrap()).unwrap())
            .collect::<Vec<CString>>();
        let mut files_ptr= files_str.iter() // do NOT into_iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let value_cstr = CString::new(value).unwrap();

        let mut result: u64 = 0;

        let error_code = checkRevision(
            value_cstr.as_ptr(),
            files_ptr.as_mut_ptr(),
            files_str.len() as i32,
            mpq_number,
            &mut result
        );

        println!("result {:?}", result as u32);

        result as u32
    }
}

pub fn check_revision_flat(value: String, file1: &Path, file2: &Path, file3: &Path, mpq_number: i32) -> u32 {
    unsafe {
        let file1_str = CString::new(file1.to_str().unwrap()).unwrap();
        let file2_str = CString::new(file2.to_str().unwrap()).unwrap();
        let file3_str = CString::new(file3.to_str().unwrap()).unwrap();

        let value_cstr = CString::new(value).unwrap();
        let mut result: u64 = 0;

        let error_code = checkRevisionFlat(
            value_cstr.as_ptr(),
            file1_str.as_ptr(),
            file2_str.as_ptr(),
            file3_str.as_ptr(),
            mpq_number,
            &mut result
        );

        result as u32
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

    #[test]
    fn test_check_revision_flat() {
        let value = String::from("B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B");
        let file1 = Path::new("../mock/war3.exe");
        let file2 = Path::new("../mock/Storm.dll");
        let file3 = Path::new("../mock/game.dll");

        assert_eq!(check_revision_flat(value, file1, file2, file3, 1), 1076278704 as u32)
    }

    #[test]
    fn test_check_revision() {
        let value = String::from("B=454282227 C=2370009462 A=2264812340 4 A=A^S B=B-C C=C-A A=A+B");
        let file1 = Path::new("../mock/war3.exe");
        let files = vec![file1];

        assert_eq!(check_revision(value, files, 1), 3796461076 as u32)
    }
}
