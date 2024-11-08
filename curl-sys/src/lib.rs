use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};
use std::slice;

extern "C" {
    fn curl_easy_init() -> *mut CURL;
    fn curl_easy_setopt(handle: *mut CURL, option: CURLoption, ...) -> CURLcode;
    fn curl_easy_perform(handle: *mut CURL) -> CURLcode;
    fn curl_easy_cleanup(handle: *mut CURL);
}

// 定义一些 libcurl 的常量和数据类型
type CURL = c_void;
type CURLoption = c_int;
type CURLcode = c_int;

const CURLOPT_URL: CURLoption = 10002;
const CURLOPT_WRITEFUNCTION: CURLoption = 20011;
const CURLOPT_WRITEDATA: CURLoption = 10001;
const CURLOPT_FOLLOWLOCATION: CURLoption = 52;

extern "C" fn write_callback(
    data: *const c_char,
    size: c_int,
    nmemb: c_int,
    userdata: *mut std::os::raw::c_void,
) -> c_int {
    let buffer = unsafe { &mut *(userdata as *mut Vec<u8>) };
    let slice = unsafe { slice::from_raw_parts(data as *const u8, (size * nmemb) as usize) };
    buffer.extend_from_slice(slice);
    size * nmemb as c_int
}

pub fn curl_get(url: String) -> String {
    unsafe {
        let curl = curl_easy_init();
        let url = CString::new(url).unwrap();
        let mut response_data = Vec::new();
        curl_easy_setopt(curl, CURLOPT_URL, url.as_ptr());
        curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1);
        curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_callback as usize);
        curl_easy_setopt(
            curl,
            CURLOPT_WRITEDATA,
            &mut response_data as *mut _ as *mut c_void,
        );

        let result = curl_easy_perform(curl);
        if result != 0 {
            return format!("libcurl error: {}", result);
        }

        curl_easy_cleanup(curl);
        String::from_utf8_lossy(&response_data).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = curl_get("http://httpbin.org/headers".to_string());
        println!("get {s}");
        assert!(s.contains("httpbin"));
    }
}
