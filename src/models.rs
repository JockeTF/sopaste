use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use chrono::NaiveDate;
use chrono::NaiveTime;

use sqlx::query_as;
use sqlx::FromRow;
use sqlx::Result;
use sqlx::Type;

use crate::storage::Pool;

#[derive(Type, Hash, PartialEq, Eq)]
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

#[derive(FromRow)]
pub struct ListRow {
    pub id: Text,
    pub name: Text,
    pub description: Text,
    pub language: Text,
    pub start: i32,
    pub password: Text,
    pub ip: Text,
    pub proxy: Text,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub checked: Option<NaiveDate>,
    pub size: Option<f64>,
    pub removed: i8,
    pub parent: Text,
}

impl ListRow {
    pub async fn find(pool: &Pool, id: &str) -> Result<Self> {
        let sql = "
            SELECT
                list.*
            FROM
                list
            WHERE
                removed IS FALSE
                AND list.id = ?
        ";

        query_as(sql).bind(id).fetch_one(&**pool).await
    }
}

#[derive(FromRow)]
pub struct TextRow {
    pub id: Text,
    pub text: Text,
}

impl TextRow {
    pub async fn find(pool: &Pool, id: &str) -> Result<Self> {
        let sql = "
            SELECT
                text.*
            FROM
                list
                INNER JOIN text ON list.id = text.id
            WHERE
                removed IS FALSE
                AND list.id = ?
        ";

        query_as(sql).bind(id).fetch_one(&**pool).await
    }
}

#[derive(FromRow)]
pub struct TreeItem {
    pub id: Text,
    pub name: Text,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub parent: Text,
}

impl TreeItem {
    pub const ROOT: &'static str = "";

    pub async fn list(pool: &Pool, id: &str) -> Result<Vec<Self>> {
        let sql = "
            WITH RECURSIVE
            cte (id, name, date, time, parent) AS (
                SELECT
                    id, name, date, time, parent
                FROM
                    list
                WHERE
                    removed IS FALSE
                    AND list.id = ?

                UNION SELECT
                    rel.id, rel.name, rel.date, rel.time, rel.parent
                FROM
                    list AS rel
                    INNER JOIN cte
                        ON cte.id = rel.parent
                        OR rel.id = cte.parent
                WHERE
                    rel.removed IS FALSE
            )
            SELECT
                id, name, date, time, parent
            FROM
                cte
            ORDER BY
                date, time, id
            LIMIT
                128
        ";

        query_as(sql).bind(id).fetch_all(&**pool).await
    }
}
