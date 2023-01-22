use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub struct MongoRepo {
    client: Client,
}

impl MongoRepo {
    pub async fn init() -> Self {
        println!("Connecting to DB");

        let client_options = ClientOptions::parse("mongodb://localhost:8081")
            .await
            .expect("Connection Error db/mod.rs");

        let client = Client::with_options(client_options).expect("with_options Error db/mod.rs");

        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("Ping Failed db/mod.rs");

        println!("Connected successfully.");
        MongoRepo { client }
    }

    pub async fn get_db(&self, db_name: &str) -> Database {
        let db_obj = self.client.database(&db_name);

        db_obj
    }

    pub async fn get_default_db(&self) -> Database {
        self.get_db("passmage").await
    }
}
