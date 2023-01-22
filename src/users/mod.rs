use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use rocket_session_store::Session;

use crate::user_db::{
    get_certain_user_doc_username, get_user_collection, get_user_obj, UserToShow, UserToStore,
};

mod support_fn;

use support_fn::{compare_passwords, get_password_hash, get_random_string};

#[get("/?<username>")]
pub async fn get_certain_user(
    db: &State<Database>,
    username: &str,
) -> Result<Json<UserToShow>, Status> {
    let users = get_user_collection(db);
    let db_result = get_certain_user_doc_username(users, &username).await;

    match db_result {
        Some(document_data) => Ok(Json(get_user_obj(document_data))),

        None => Err(Status::NotFound),
    }
}

#[post("/", data = "<input>")]
pub async fn add_user(db: &State<Database>, input: Json<UserToStore>) -> Result<String, Status> {
    let users = get_user_collection(db);
    let db_result = get_certain_user_doc_username(users, &input.username).await;

    match db_result {
        Some(_) => Err(Status::MethodNotAllowed),
        None => {
            let mut raw_password = input.password.clone();

            let secret_phrase = get_random_string();
            raw_password.push_str(&secret_phrase);

            let password = get_password_hash(raw_password);

            let users = get_user_collection(db);
            users
                .insert_one(
                    doc! {
                        "username":input.username.clone(),
                        "password":password,
                        "secret":secret_phrase
                    },
                    None,
                )
                .await
                .expect("Add user to DB error");

            Ok("add_user".to_string())
        }
    }
}

#[post("/login", data = "<input>")]
pub async fn login_user(
    db: &State<Database>,
    input: Json<UserToStore>,
    session: Session<'_, String>,
) -> Result<String, Status> {
    let users = get_user_collection(db);
    let db_result = get_certain_user_doc_username(users, &input.username).await;

    match db_result {
        Some(user_data) => {
            println!("Logging");
            if compare_passwords(user_data.clone(), input.password.clone()) {
                let user_object = get_user_obj(user_data);
                session.set(user_object.id).await.expect("Setting session");
                println!("Login");
                Ok("Login".to_string())
            } else {
                Err(Status::Unauthorized)
            }
        }
        None => Err(Status::NotFound),
    }
}

#[get("/logout")]
pub async fn logout_user(session: Session<'_, String>) -> Result<String, Status> {
    session.remove().await.expect("Logout");
    Ok("logout".to_string())
}
