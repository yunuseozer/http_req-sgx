// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::panic;
use sgx_tunittest::*;

extern crate http_req;
extern crate unicase;
mod uri;
mod request;
mod response;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
uri::remove_space,
uri::uri_full_parse,
uri::uri_parse,
uri::uri_scheme,
uri::uri_uesr_info,
uri::uri_host,
uri::uri_host_header,
uri::uri_port,
uri::uri_corr_port,
uri::uri_path,
uri::uri_query,
uri::uri_fragment,
uri::uri_resource,
uri::uri_display,
uri::authority_username,
uri::authority_password,
uri::authority_host,
uri::authority_port,
uri::authority_from_str,
uri::authority_display,
uri::range_c_new,
uri::range_from_range_c,
uri::range_c_index,
response::status_code_new,
response::status_code_info,
response::status_code_success,
response::status_code_redirect,
response::status_code_client_err,
response::status_code_server_err,
response::status_code_is,
response::status_code_reason,
response::status_code_from,
response::u16_from_status_code,
response::status_code_display,
response::status_code_from_str,
response::status_from,
response::status_from_str,
response::headers_new,
response::headers_get,
response::headers_insert,
response::headers_default_http,
response::headers_from_str,
response::headers_from,
response::headers_case_insensitive,
response::hash_map_from_headers,
response::find_slice_e,
response::res_from_head,
response::res_try_from,
response::res_status_code,
response::res_version,
response::res_reason,
response::res_headers,
response::res_content_len,
response::res_body,
|| should_panic!(response::res_from_empty()),
request::copy_data_until,
request::method_display,
request::request_b_new,
request::request_b_method,
request::request_b_headers,
request::request_b_header,
request::request_b_body,
request::request_b_send,
request::request_b_send_secure,
request::request_b_parse_msg,
request::request_new,
request::request_method,
request::request_headers,
request::request_header,
request::request_body,
request::request_connect_timeout,
//request::request_read_timeout,
request::request_write_timeout,
request::request_send,
request::request_get,
request::request_head,
);
    sgx_status_t::SGX_SUCCESS
}
