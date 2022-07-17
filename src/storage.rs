use rocket::async_trait;
use rocket::figment::Figment;

use sea_orm::Database as Connector;
use sea_orm::DbConn;
use sea_orm::DbErr;
use sea_orm_rocket::Config;
use sea_orm_rocket::Database;
use sea_orm_rocket::Pool as SeaPool;

#[derive(Database, Debug)]
#[database("pastebin")]
pub struct Pool(Wrapper);

impl Pool {
    pub fn conn(&self) -> &DbConn {
        &self.0.0
    }
}

#[derive(Debug)]
pub struct Wrapper(DbConn);

#[async_trait]
impl SeaPool for Wrapper {
    type Connection = DbConn;
    type Error = DbErr;

    // https://github.com/SeaQL/sea-orm/tree/master/examples/rocket_example
    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config = figment.extract::<Config>().unwrap();
        let connection = Connector::connect(config.url).await?;

        Ok(Wrapper(connection))
    }

    fn borrow(&self) -> &Self::Connection {
        &self.0
    }
}
