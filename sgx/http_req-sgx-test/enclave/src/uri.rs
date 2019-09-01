use std::prelude::v1::*;
use http_req::uri::*;
use std::ops::Range;

fn remove_spaces(text: &mut String) {
    text.retain(|c| !c.is_whitespace());
}

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

const TEST_URIS: [&str; 5] = [
    "https://user:info@foo.com:12/bar/baz?query#fragment",
    "file:///C:/Users/User/Pictures/screenshot.png",
    "https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol",
    "mailto:John.Doe@example.com",
    "https://[4b10:bbb0:0:d0::ba7:8001]:443/",
];

const TEST_AUTH: [&str; 4] = [
    "user:info@foo.com:12",
    "en.wikipedia.org",
    "John.Doe@example.com",
    "[4b10:bbb0:0:d0::ba7:8001]:443",
];

//#[test]
pub fn remove_space() {
    let mut text = String::from("Hello World     !");
    let expect = String::from("HelloWorld!");

    remove_spaces(&mut text);
    assert_eq!(text, expect);
}

//#[test]
pub fn uri_full_parse() {
    let uri = "abc://username:password@example.com:123/path/data?key=value&key2=value2#fragid1"
        .parse::<Uri>()
        .unwrap();
    assert_eq!(uri.scheme(), "abc");

    assert_eq!(uri.user_info(), Some("username:password"));
    assert_eq!(uri.host(), Some("example.com"));
    assert_eq!(uri.port(), Some(123));

    assert_eq!(uri.path(), Some("/path/data"));
    assert_eq!(uri.query(), Some("key=value&key2=value2"));
    assert_eq!(uri.fragment(), Some("fragid1"));
}

//#[test]
pub fn uri_parse() {
    for uri in TEST_URIS.iter() {
        uri.parse::<Uri>().unwrap();
    }
}

//#[test]
pub fn uri_scheme() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].scheme(), "https");
    assert_eq!(uris[1].scheme(), "file");
    assert_eq!(uris[2].scheme(), "https");
    assert_eq!(uris[3].scheme(), "mailto");
    assert_eq!(uris[4].scheme(), "https");
}

//#[test]
pub fn uri_uesr_info() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].user_info(), Some("user:info"));
    assert_eq!(uris[1].user_info(), None);
    assert_eq!(uris[2].user_info(), None);
    assert_eq!(uris[3].user_info(), None);
    assert_eq!(uris[4].user_info(), None);
}

//#[test]
pub fn uri_host() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].host(), Some("foo.com"));
    assert_eq!(uris[1].host(), None);
    assert_eq!(uris[2].host(), Some("en.wikipedia.org"));
    assert_eq!(uris[3].host(), None);
    assert_eq!(uris[4].host(), Some("[4b10:bbb0:0:d0::ba7:8001]"));
}

//#[test]
pub fn uri_host_header() {
    let uri_def: Uri = "https://en.wikipedia.org:443/wiki/Hypertext_Transfer_Protocol"
        .parse()
        .unwrap();
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].host_header(), Some("foo.com:12".to_string()));
    assert_eq!(uris[2].host_header(), Some("en.wikipedia.org".to_string()));
    assert_eq!(uri_def.host_header(), Some("en.wikipedia.org".to_string()));
}

//#[test]
pub fn uri_port() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].port(), Some(12));
    assert_eq!(uris[4].port(), Some(443));

    for i in 1..3 {
        assert_eq!(uris[i].port(), None);
    }
}

//#[test]
pub fn uri_corr_port() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].corr_port(), 12);
    assert_eq!(uris[1].corr_port(), HTTP_PORT);
    assert_eq!(uris[2].corr_port(), HTTPS_PORT);
    assert_eq!(uris[3].corr_port(), HTTP_PORT);
    assert_eq!(uris[4].corr_port(), HTTPS_PORT);
}

//#[test]
pub fn uri_path() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].path(), Some("/bar/baz"));
    assert_eq!(
        uris[1].path(),
        Some("/C:/Users/User/Pictures/screenshot.png")
    );
    assert_eq!(uris[2].path(), Some("/wiki/Hypertext_Transfer_Protocol"));
    assert_eq!(uris[3].path(), Some("John.Doe@example.com"));
    assert_eq!(uris[4].path(), None);
}

//#[test]
pub fn uri_query() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].query(), Some("query"));

    for i in 1..4 {
        assert_eq!(uris[i].query(), None);
    }
}

//#[test]
pub fn uri_fragment() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].fragment(), Some("fragment"));

    for i in 1..4 {
        assert_eq!(uris[i].fragment(), None);
    }
}

//#[test]
pub fn uri_resource() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(uris[0].resource(), "/bar/baz?query#fragment");
    assert_eq!(uris[1].resource(), "/C:/Users/User/Pictures/screenshot.png");
    assert_eq!(uris[2].resource(), "/wiki/Hypertext_Transfer_Protocol");
    assert_eq!(uris[3].resource(), "John.Doe@example.com");
    assert_eq!(uris[4].resource(), "/");
}

//#[test]
pub fn uri_display() {
    let uris: Vec<_> = TEST_URIS
        .iter()
        .map(|uri| uri.parse::<Uri>().unwrap())
        .collect();

    assert_eq!(
        uris[0].to_string(),
        "https://user:****@foo.com:12/bar/baz?query#fragment"
    );

    for i in 1..uris.len() {
        let s = uris[i].to_string();
        assert_eq!(s, TEST_URIS[i]);
    }
}

//#[test]
pub fn authority_username() {
    let auths: Vec<_> = TEST_AUTH
        .iter()
        .map(|auth| auth.parse::<Authority>().unwrap())
        .collect();

    assert_eq!(auths[0].username(), Some("user"));
    assert_eq!(auths[1].username(), None);
    assert_eq!(auths[2].username(), Some("John.Doe"));
    assert_eq!(auths[3].username(), None);
}

//#[test]
pub fn authority_password() {
    let auths: Vec<_> = TEST_AUTH
        .iter()
        .map(|auth| auth.parse::<Authority>().unwrap())
        .collect();

    assert_eq!(auths[0].password(), Some("info"));
    assert_eq!(auths[1].password(), None);
    assert_eq!(auths[2].password(), None);
    assert_eq!(auths[3].password(), None);
}

//#[test]
pub fn authority_host() {
    let auths: Vec<_> = TEST_AUTH
        .iter()
        .map(|auth| auth.parse::<Authority>().unwrap())
        .collect();

    assert_eq!(auths[0].host(), "foo.com");
    assert_eq!(auths[1].host(), "en.wikipedia.org");
    assert_eq!(auths[2].host(), "example.com");
    assert_eq!(auths[3].host(), "[4b10:bbb0:0:d0::ba7:8001]");
}

//#[test]
pub fn authority_port() {
    let auths: Vec<_> = TEST_AUTH
        .iter()
        .map(|auth| auth.parse::<Authority>().unwrap())
        .collect();

    assert_eq!(auths[0].port(), Some(12));
    assert_eq!(auths[1].port(), None);
    assert_eq!(auths[2].port(), None);
    assert_eq!(auths[3].port(), Some(443));
}

//#[test]
pub fn authority_from_str() {
    for auth in TEST_AUTH.iter() {
        auth.parse::<Authority>().unwrap();
    }
}

//#[test]
pub fn authority_display() {
    let auths: Vec<_> = TEST_AUTH
        .iter()
        .map(|auth| auth.parse::<Authority>().unwrap())
        .collect();

    assert_eq!("user:****@foo.com:12", auths[0].to_string());

    for i in 1..auths.len() {
        let s = auths[i].to_string();
        assert_eq!(s, TEST_AUTH[i]);
    }
}

//#[test]
pub fn range_c_new() {
    assert_eq!(
        RangeC::new(22, 343),
        RangeC {
            start: 22,
            end: 343,
        }
    )
}

//#[test]
pub fn range_from_range_c() {
    assert_eq!(
        Range::from(RangeC::new(222, 43)),
        Range {
            start: 222,
            end: 43,
        }
    )
}

//#[test]
pub fn range_c_index() {
    const RANGE: RangeC = RangeC::new(0, 4);
    let text = TEST_AUTH[0].to_string();

    assert_eq!(text[..4], text[RANGE])
}
