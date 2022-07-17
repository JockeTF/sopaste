use sea_orm::entity::prelude::*;

use chrono::NaiveDate;
use chrono::NaiveTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "list")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    pub description: String,
    pub language: String,
    pub start: i32,
    pub password: String,
    pub ip: String,
    pub proxy: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub checked: Option<NaiveDate>,
    pub size: Option<f64>,
    pub removed: i8,
    pub parent: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
