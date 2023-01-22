use mongodb::Database;
use rocket::State;
use rocket_session_store::Session;

use crate::user_db::get_certain_user_doc_id;

#[get("/")]
pub async fn index(session: Session<'_, String>, db: &State<Database>) -> String {
    let name: Option<String> = session.get().await.expect("Get session Error");
    if let Some(name) = name {
        let db_result = get_certain_user_doc_id(db, &name).await;

        match db_result {
            Some(document_data) => {
                let username = document_data
                    .get_str("username")
                    .expect("User ID unwrap error")
                    .to_string();
                format!("Hello, {}!", username)
            }

            None => format!("Hello, Error!"),
        }
    } else {
        "Hello, world!".into()
    }
}
