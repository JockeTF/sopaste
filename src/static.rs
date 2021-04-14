use std::io::Cursor;

use crc::Crc;
use crc::CRC_64_WE;

use rocket::http::hyper::header;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::Result;
use rocket::*;

struct Static {
    body: &'static [u8],
    ctype: ContentType,
    digest: u64,
}

impl Static {
    const fn new(ctype: ContentType, body: &'static [u8]) -> Self {
        let digest = Crc::<u64>::new(&CRC_64_WE).checksum(body);

        Static {
            body,
            ctype,
            digest,
        }
    }
}

impl<'r> Responder<'r, 'static> for Static {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        let mut response = Response::build();

        let etag = header::ETAG.as_str();
        let if_none_match = header::IF_NONE_MATCH.as_str();
        let cache_control = header::CACHE_CONTROL.as_str();

        let digest = request
            .headers()
            .get(if_none_match)
            .map(|header| header.parse())
            .find_map(|value| value.ok());

        response.header(self.ctype);
        response.raw_header(etag, self.digest.to_string());

        if Some(self.digest) == digest {
            response.status(Status::NotModified);
        } else {
            response.sized_body(self.body.len(), Cursor::new(self.body));
        };

        if cfg!(debug_assertions) {
            response.raw_header(cache_control, "no-cache");
        } else {
            response.raw_header(cache_control, "max-age=300");
        }

        response.ok()
    }
}

#[get("/logo.png")]
fn logo() -> Static {
    Static::new(ContentType::PNG, include_bytes!("../static/logo.png"))
}

#[get("/shadow.png")]
fn shadow() -> Static {
    Static::new(ContentType::PNG, include_bytes!("../static/shadow.png"))
}

#[get("/style.css")]
fn style() -> Static {
    Static::new(ContentType::CSS, include_bytes!("../static/style.css"))
}

pub fn routes() -> Vec<Route> {
    routes![logo, shadow, style]
}
