use dotenv::dotenv;
use std::env;
use std::sync::Mutex;
use diesel::connection::Connection;
use diesel::pg::PgConnection;

lazy_static! {
    pub static ref DB_CONNECTION: Mutex<PgConnection> = {
        dotenv().ok();

        let db = create_connection();
        return Mutex::new(db);
    };
}

fn create_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url);

    return connection.unwrap();
}