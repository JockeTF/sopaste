use std::iter::once;

use axum::headers::ETag;
use axum::headers::Header;
use axum::headers::IfNoneMatch;
use axum::http::header;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use axum::TypedHeader;

use const_fnv1a_hash::fnv1a_hash_64;
use const_format::formatcp;

use crate::state::AppState;

struct StaticFile {
    body: &'static [u8],
    ctype: &'static str,
    etag: &'static str,
}

impl StaticFile {
    const fn new(body: &'static [u8], ctype: &'static str, etag: &'static str) -> Self {
        StaticFile { body, ctype, etag }
    }

    fn response(&self, tags: &IfNoneMatch) -> Response {
        let ctype = HeaderValue::from_static(self.ctype);
        let etag = HeaderValue::from_static(self.etag);
        let dec = ETag::decode(&mut once(&etag)).unwrap();

        let mut response = if tags.precondition_passes(&dec) {
            self.body.into_response()
        } else {
            StatusCode::NOT_MODIFIED.into_response()
        };

        let cache = if cfg!(debug_assertions) {
            HeaderValue::from_static("no-cache")
        } else {
            HeaderValue::from_static("max-age=300")
        };

        let headers = response.headers_mut();
        headers.insert(header::CACHE_CONTROL, cache);
        headers.insert(header::CONTENT_TYPE, ctype);
        headers.insert(header::ETAG, etag);

        response
    }
}

macro_rules! file {
    ($file:literal, $type:literal) => {{
        const DATA: &[u8] = include_bytes!(concat!("../static/", $file));
        const ETAG: &str = formatcp!("\"{}\"", fnv1a_hash_64(DATA, Some(65536)));
        const FILE: StaticFile = StaticFile::new(DATA, $type, ETAG);

        &FILE
    }};
}

async fn favicon(TypedHeader(tags): TypedHeader<IfNoneMatch>) -> Response {
    file!("favicon.svg", "image/svg+xml").response(&tags)
}

async fn license(TypedHeader(tags): TypedHeader<IfNoneMatch>) -> Response {
    file!("../license.txt", "text/plain; charset=utf-8").response(&tags)
}

async fn robots(TypedHeader(tags): TypedHeader<IfNoneMatch>) -> Response {
    file!("robots.txt", "text/plain; charset=utf-8").response(&tags)
}

async fn style(TypedHeader(tags): TypedHeader<IfNoneMatch>) -> Response {
    file!("style.css", "text/css; charset=utf-8").response(&tags)
}

#[cfg(not(feature = "source"))]
use crate::result::fallback as source;

#[cfg(feature = "source")]
async fn source(TypedHeader(tags): TypedHeader<IfNoneMatch>) -> Response {
    file!("sopaste.tar.xz", "application/x-xz-compressed-tar").response(&tags)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/favicon.svg", get(favicon))
        .route("/license.txt", get(license))
        .route("/robots.txt", get(robots))
        .route("/sopaste.tar.xz", get(source))
        .route("/style.css", get(style))
}
