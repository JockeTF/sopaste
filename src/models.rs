use rocket_contrib::database;
use rocket_contrib::databases::diesel::MysqlConnection;

#[database("pastebin")]
pub struct Pool(MysqlConnection);
