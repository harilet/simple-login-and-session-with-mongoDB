use rocket_session_store::{memory::MemoryStore, CookieConfig, SessionStore};

use std::time::Duration;

#[macro_use]
extern crate rocket;

mod db;
mod default;
mod user_db;
mod users;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Instance a store that fits your needs and wrap it in a Box in SessionStore.
    let memory_store: MemoryStore<String> = MemoryStore::default();
    let store: SessionStore<String> = SessionStore {
        store: Box::new(memory_store),
        name: "token".into(),
        duration: Duration::from_secs(3600 * 24 * 3),
        // The cookie config is used to set the cookie's path and other options.
        cookie: CookieConfig::default(),
    };

    println!("Server Starting");

    let mongo_repo = db::MongoRepo::init().await;
    let db = mongo_repo.get_default_db().await;

    let _rocket = rocket::build()
        .attach(store.fairing())
        .mount("/", routes![default::index, users::login_user])
        .mount(
            "/users",
            routes![users::add_user, users::get_certain_user, users::logout_user],
        )
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
