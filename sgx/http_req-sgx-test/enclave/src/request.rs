use std::prelude::v1::*;
use http_req::request::*;
use http_req::response::*;
use http_req::tls;
use http_req::uri::Uri;
use http_req::{error::Error, response::StatusCode};
use std::net::TcpStream;
use std::io::{self, Cursor};
use std::time::Duration;
const CR_LF: &str = "\r\n";
const CR_LF_2: [u8; 4] = [13, 10, 13, 10];

const UNSUCCESS_CODE: StatusCode = StatusCode::new(400);
const URI: &str = "http://doc.rust-lang.org/std/string/index.html";
const URI_S: &str = "https://doc.rust-lang.org/std/string/index.html";
const BODY: [u8; 14] = [78, 97, 109, 101, 61, 74, 97, 109, 101, 115, 43, 74, 97, 121];

const RESPONSE: &[u8; 129] = b"HTTP/1.1 200 OK\r\n\
                                     Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                                     Content-Type: text/html\r\n\
                                     Content-Length: 100\r\n\r\n\
                                     <html>hello</html>\r\n\r\nhello";

const RESPONSE_H: &[u8; 102] = b"HTTP/1.1 200 OK\r\n\
                                       Date: Sat, 11 Jan 2003 02:44:04 GMT\r\n\
                                       Content-Type: text/html\r\n\
                                       Content-Length: 100\r\n\r\n";

//#[test]
pub fn copy_data_until() {
    let mut reader = Vec::new();
    reader.extend(&RESPONSE[..]);

    let mut reader = Cursor::new(reader);
    let mut writer = Vec::new();

    copy_until(&mut reader, &mut writer, &CR_LF_2).unwrap();
    assert_eq!(writer, &RESPONSE_H[..]);
}

//#[test]
pub fn method_display() {
    const METHOD: Method = Method::HEAD;
    assert_eq!(&format!("{}", METHOD), "HEAD");
}

//#[test]
pub fn request_b_new() {
    RequestBuilder::new(&URI.parse().unwrap());
    RequestBuilder::new(&URI_S.parse().unwrap());
}

//#[test]
pub fn request_b_method() {
    let uri: Uri = URI.parse().unwrap();
    let mut req = RequestBuilder::new(&uri);
    let req = req.method(Method::HEAD);

    assert_eq!(req.method, Method::HEAD);
}

//#[test]
pub fn request_b_headers() {
    let mut headers = Headers::new();
    headers.insert("Accept-Charset", "utf-8");
    headers.insert("Accept-Language", "en-US");
    headers.insert("Host", "doc.rust-lang.org");
    headers.insert("Connection", "Close");

    let uri: Uri = URI.parse().unwrap();
    let mut req = RequestBuilder::new(&uri);
    let req = req.headers(headers.clone());

    assert_eq!(req.headers, headers);
}

//#[test]
pub fn request_b_header() {
    let uri: Uri = URI.parse().unwrap();
    let mut req = RequestBuilder::new(&uri);
    let k = "Connection";
    let v = "Close";

    let mut expect_headers = Headers::new();
    expect_headers.insert("Host", "doc.rust-lang.org");
    expect_headers.insert("Referer", "http://doc.rust-lang.org/std/string/index.html");
    expect_headers.insert(k, v);

    let req = req.header(k, v);

    assert_eq!(req.headers, expect_headers);
}

//#[test]
pub fn request_b_body() {
    let uri: Uri = URI.parse().unwrap();
    let mut req = RequestBuilder::new(&uri);
    let req = req.body(&BODY);

    assert_eq!(req.body, Some(BODY.as_ref()));
}

//#[ignore]
//#[test]
pub fn request_b_send() {
    let mut writer = Vec::new();
    let uri: Uri = URI.parse().unwrap();
    let mut stream = TcpStream::connect((uri.host().unwrap_or(""), uri.corr_port())).unwrap();

    RequestBuilder::new(&URI.parse().unwrap())
        .header("Connection", "Close")
        .send(&mut stream, &mut writer)
        .unwrap();
}

//#[ignore]
//#[test]
pub fn request_b_send_secure() {
    let mut writer = Vec::new();
    let uri: Uri = URI_S.parse().unwrap();

    let stream = TcpStream::connect((uri.host().unwrap_or(""), uri.corr_port())).unwrap();
    let mut secure_stream = tls::Config::default()
        .connect(uri.host().unwrap_or(""), stream)
        .unwrap();

    RequestBuilder::new(&URI_S.parse().unwrap())
        .header("Connection", "Close")
        .send(&mut secure_stream, &mut writer)
        .unwrap();
}

//#[test]
pub fn request_b_parse_msg() {
    let uri = URI.parse().unwrap();
    let req = RequestBuilder::new(&uri);

    const DEFAULT_MSG: &str = "GET /std/string/index.html HTTP/1.1\r\n\
                               Referer: http://doc.rust-lang.org/std/string/index.html\r\n\
                               Host: doc.rust-lang.org\r\n\r\n";
    let msg = req.parse_msg();
    let msg = String::from_utf8_lossy(&msg).into_owned();

    for line in DEFAULT_MSG.lines() {
        assert!(msg.contains(line));
    }

    for line in msg.lines() {
        assert!(DEFAULT_MSG.contains(line));
    }
}

//#[test]
pub fn request_new() {
    let uri = URI.parse().unwrap();
    Request::new(&uri);
}

//#[test]
pub fn request_method() {
    let uri = URI.parse().unwrap();
    let mut req = Request::new(&uri);
    req.method(Method::HEAD);

    assert_eq!(req.inner.method, Method::HEAD);
}

//#[test]
pub fn request_headers() {
    let mut headers = Headers::new();
    headers.insert("Accept-Charset", "utf-8");
    headers.insert("Accept-Language", "en-US");
    headers.insert("Host", "doc.rust-lang.org");
    headers.insert("Connection", "Close");

    let uri: Uri = URI.parse().unwrap();
    let mut req = Request::new(&uri);
    let req = req.headers(headers.clone());

    assert_eq!(req.inner.headers, headers);
}

//#[test]
pub fn request_header() {
    let uri: Uri = URI.parse().unwrap();
    let mut req = Request::new(&uri);
    let k = "Accept-Language";
    let v = "en-US";

    let mut expect_headers = Headers::new();
    expect_headers.insert("Host", "doc.rust-lang.org");
    expect_headers.insert("Referer", "http://doc.rust-lang.org/std/string/index.html");
    expect_headers.insert("Connection", "Close");
    expect_headers.insert(k, v);

    let req = req.header(k, v);

    assert_eq!(req.inner.headers, expect_headers);
}

//#[test]
pub fn request_body() {
    let uri = URI.parse().unwrap();
    let mut req = Request::new(&uri);
    let req = req.body(&BODY);

    assert_eq!(req.inner.body, Some(BODY.as_ref()));
}

//#[test]
pub fn request_connect_timeout() {
    let uri = URI.parse().unwrap();
    let mut request = Request::new(&uri);
    request.connect_timeout(Some(Duration::from_nanos(1)));

    assert_eq!(request.connect_timeout, Some(Duration::from_nanos(1)));

    let err = request.send(&mut io::sink()).unwrap_err();
    match err {
        Error::IO(err) => assert_eq!(err.kind(), io::ErrorKind::TimedOut),
        other => panic!("Expected error to be io::Error, got: {:?}", other),
    };
}

//#[test]
pub fn request_read_timeout() {
    let uri = URI.parse().unwrap();
    let mut request = Request::new(&uri);
    request.read_timeout(Some(Duration::from_nanos(1)));

    assert_eq!(request.read_timeout, Some(Duration::from_nanos(1)));

    let err = request.send(&mut io::sink()).unwrap_err();
    match err {
        Error::IO(err) => match err.kind() {
            io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut => {}
            other => panic!(
                "Expected error kind to be one of WouldBlock/TimedOut, got: {:?}",
                other
            ),
        },
        other => panic!("Expected error to be io::Error, got: {:?}", other),
    };
}

//#[test]
pub fn request_write_timeout() {
    let uri = URI.parse().unwrap();
    let mut request = Request::new(&uri);
    request.write_timeout(Some(Duration::from_nanos(100)));

    assert_eq!(request.write_timeout, Some(Duration::from_nanos(100)));
}

//#[test]
pub fn request_send() {
    let mut writer = Vec::new();
    let uri = URI.parse().unwrap();
    let res = Request::new(&uri).send(&mut writer).unwrap();

    assert_ne!(res.status_code(), UNSUCCESS_CODE);
}

//#[ignore]
//#[test]
pub fn request_get() {
    let mut writer = Vec::new();
    let res = get(URI, &mut writer).unwrap();

    assert_ne!(res.status_code(), UNSUCCESS_CODE);

    let mut writer = Vec::with_capacity(200);
    let res = get(URI_S, &mut writer).unwrap();

    assert_ne!(res.status_code(), UNSUCCESS_CODE);
}

//#[ignore]
//#[test]
pub fn request_head() {
    let res = head(URI).unwrap();
    assert_ne!(res.status_code(), UNSUCCESS_CODE);

    let res = head(URI_S).unwrap();
    assert_ne!(res.status_code(), UNSUCCESS_CODE);
}
