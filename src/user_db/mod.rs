use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Collection, Database,
};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UserToStore {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct DBUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub secret: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct UserToShow {
    pub username: String,
    pub id: String,
}

pub fn get_user_collection(db: &State<Database>) -> Collection<Document> {
    db.collection::<Document>("users")
}

pub fn get_user_obj(user: Document) -> UserToShow {
    UserToShow {
        username: user
            .get_str("username")
            .expect("User username unwrap error")
            .to_string(),
        id: user
            .get_object_id("_id")
            .expect("User ID unwrap error")
            .to_hex(),
    }
}

pub async fn get_certain_user_doc_username(
    users: Collection<Document>,
    username: &str,
) -> Option<mongodb::bson::Document> {
    users
        .find_one(
            doc! {
                "username":username,
            },
            None,
        )
        .await
        .expect("Get user from DB error")
}

pub async fn get_certain_user_doc_id(
    db: &State<Database>,
    id: &str,
) -> Option<mongodb::bson::Document> {
    let users = get_user_collection(db);
    let user_id_object = ObjectId::parse_str(id).expect("msg");

    users
        .find_one(
            doc! {
                "_id": user_id_object,
            },
            None,
        )
        .await
        .expect("Get user from DB error")
}
