use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::slice;

extern "C" {
    fn curl_easy_init() -> *mut CURL;
    fn curl_easy_setopt(handle: *mut CURL, option: CURLoption, ...) -> CURLcode;
    fn curl_easy_perform(handle: *mut CURL) -> CURLcode;
    fn curl_easy_cleanup(handle: *mut CURL);
    fn curl_easy_strerror(code: CURLcode) -> *const c_char;

}

// 定义一些 libcurl 的常量和数据类型
type CURL = c_void;
type CURLoption = c_int;
type CURLcode = c_int;

const CURLOPT_URL: CURLoption = 10002;
const CURLOPT_WRITEFUNCTION: CURLoption = 20011;
const CURLOPT_WRITEDATA: CURLoption = 10001;
const CURLOPT_FOLLOWLOCATION: CURLoption = 52;
const CURLOPT_VERBOSE: CURLoption = 41;

extern "C" fn write_callback(
    data: *const c_char,
    size: c_int,
    nmemb: c_int,
    userdata: *mut std::os::raw::c_void,
) -> c_int {
    let buffer = unsafe { &mut *(userdata as *mut Vec<u8>) };
    let effective_size = size * nmemb;
    let slice = unsafe { slice::from_raw_parts(data as *const u8, effective_size as usize) };
    buffer.extend_from_slice(slice);
    effective_size as c_int
}

// 定义一个函数来获取错误信息
unsafe fn get_error_message(code: CURLcode) -> String {
    let error_ptr = curl_easy_strerror(code);
    CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
}

pub fn curl_get(url: &str, allow_redirect: bool, verbose: bool) -> Result<String, String> {
    let mut response_data = Vec::new();
    let follow_location: c_int = if allow_redirect { 1 } else { 0 };
    let verbose: c_int = if verbose { 1 } else { 0 };
    unsafe {
        let url = CString::new(url).map_err(|_| "url should not contains nul byte")?;
        let curl = curl_easy_init();
        curl_easy_setopt(curl, CURLOPT_URL, url.as_ptr());
        curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, follow_location);
        curl_easy_setopt(curl, CURLOPT_VERBOSE, verbose);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback as usize);
        curl_easy_setopt(
            curl,
            CURLOPT_WRITEDATA,
            &mut response_data as *mut _ as *mut c_void,
        );

        let result = curl_easy_perform(curl);
        curl_easy_cleanup(curl);
        if result != 0 {
            return Err(get_error_message(result));
        }
    }

    Ok(String::from_utf8_lossy(&response_data).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = dbg!(curl_get("http://httpbin.org/headers", false).unwrap());
        assert!(result.contains("httpbin"));
        let result = dbg!(curl_get("http://baidu.com", true).unwrap());
        assert!(result.contains("baidu"));
    }
}
