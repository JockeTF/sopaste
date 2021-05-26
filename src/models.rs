use chrono::NaiveDate;
use chrono::NaiveTime;

use sqlx::query_as;
use sqlx::FromRow;
use sqlx::Result;

use crate::storage::Pool;

#[derive(FromRow)]
pub struct ListRow {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub start: i32,
    pub password: Option<String>,
    pub ip: Option<String>,
    pub proxy: Option<String>,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub checked: Option<NaiveDate>,
    pub size: Option<f64>,
    pub removed: i8,
    pub parent: Option<String>,
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
    pub id: String,
    pub text: Option<String>,
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
