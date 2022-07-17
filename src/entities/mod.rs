use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use sqlx::Type;

pub mod list;
pub mod text;

#[derive(Type, Clone, Debug, Hash, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct Text(Option<Vec<u8>>);

impl Text {
    pub fn decode(&self) -> Cow<'_, str> {
        match &self.0 {
            Some(v) => String::from_utf8_lossy(v),
            None => Cow::from(""),
        }
    }

    pub fn is_blank(&self) -> bool {
        self.decode().trim().is_empty()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.decode().fmt(f)
    }
}
