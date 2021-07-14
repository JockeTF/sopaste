use std::io::Cursor;

use crc::Crc;
use crc::CRC_64_WE;

use rocket::http::hyper::header;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Result;
use rocket::*;

struct StaticFile {
    body: &'static [u8],
    ctype: ContentType,
    digest: u64,
}

impl StaticFile {
    const fn new(ctype: ContentType, body: &'static [u8]) -> Self {
        let digest = Crc::<u64>::new(&CRC_64_WE).checksum(body);

        StaticFile {
            body,
            ctype,
            digest,
        }
    }
}

impl<'r> Responder<'r, 'static> for &StaticFile {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        let mut response = Response::build();

        let etag = header::ETAG.as_str();
        let ctype = header::CONTENT_TYPE.as_str();
        let if_none_match = header::IF_NONE_MATCH.as_str();
        let cache_control = header::CACHE_CONTROL.as_str();

        let digest = request
            .headers()
            .get(if_none_match)
            .map(|header| header.parse())
            .find_map(|value| value.ok());

        response.raw_header(ctype, self.ctype.to_string());
        response.raw_header(etag, self.digest.to_string());

        if Some(self.digest) == digest {
            response.status(Status::NotModified);
        } else {
            response.sized_body(self.body.len(), Cursor::new(self.body));
        }

        if cfg!(debug_assertions) {
            response.raw_header(cache_control, "no-cache");
        } else {
            response.raw_header(cache_control, "max-age=300");
        }

        response.ok()
    }
}

macro_rules! include_static {
    ($type:ident, $file:literal) => {{
        const REF: &StaticFile = &StaticFile::new(
            rocket::http::ContentType::$type,
            include_bytes!(concat!("../static/", $file)),
        );

        REF
    }};
}

#[get("/favicon.svg")]
const fn favicon() -> &'static StaticFile {
    include_static!(SVG, "favicon.svg")
}

#[get("/logo.png")]
const fn logo() -> &'static StaticFile {
    include_static!(PNG, "logo.png")
}

#[get("/shadow.png")]
const fn shadow() -> &'static StaticFile {
    include_static!(PNG, "shadow.png")
}

#[get("/style.css")]
const fn style() -> &'static StaticFile {
    include_static!(CSS, "style.css")
}

pub fn routes() -> Vec<Route> {
    routes![favicon, logo, shadow, style]
}
