// #[macro_use]
// extern crate diesel_codegen;
// #[macro_use]
// extern crate diesel;
// extern crate dotenv;
// extern crate r2d2;
// extern crate r2d2_diesel;

// pub mod schema;
// pub mod models;

// use diesel::pg::PgConnection;
// use r2d2::Pool;
// use r2d2_diesel::ConnectionManager;
// use dotenv::dotenv;
// use std::env;

// pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     let manager = ConnectionManager::<PgConnection>::new(database_url);
//     Pool::builder().build(manager).expect("Failed to create pool.")
// }
