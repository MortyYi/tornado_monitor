use std::string::String;
use std::option::Option;
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::{doc, Document};

struct Event<'a> {
    address: &'a str,
    txn_hash: &'a str,
    commitment_or_nullifierHash: &'a str,
}
struct TornadoInOrOut<'a> {
    in_or_output: &'a Event<'a>,
    potential_in_or_outputs: Vec<&'a Event<'a>>,
}

pub struct DB<'a> {
    client_url: &'a str,
    db_name: &'a str,
    client: Client,
    db: Database,
}

impl <'a> DB<'a> {
    async fn init() -> DB<'a> {
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse("mongodb://10.20.30.97:27017").await.unwrap();

        // Manually set an option.
        client_options.app_name = Some("tornado".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options).unwrap();
        
        // Get a handle to a database.
        let db = client.database("tornado");

        Self {
            client_url: "mongodb://10.20.30.97:27017",
            db_name: "tornado",
            client: client,
            db: db,
        }
    }
    async fn get_db_names(self) -> Option<Vec<String>> {
        let result: Vec<String>;
        for db_name in self.client.list_database_names(None, None).await {
            // println!("{}", db_name);
            result = result.push(db_name)
        }
        Some(result)
    }
    fn read(self, collection_name: &str, docs: Document) -> Result<Vec<TornadoInOrOut>, mongodb::error::Error> {
        let result: Vec<TornadoInOrOut>;
        let collection = self.db.collection::<Event>(collection_name);
        let cursor = collection.find(doc! { "author": "George Orwell" }, None);
        // for cursor_result in cursor.await? {
        //     println!("title: {}", cursor_result.title);
        //     result = result.push(cursor_result)
        // }
        while let Some(book) = cursor.try_next().await? {
            println!("title: {}", book.title);
        }
        result = cursor.await?;
        Ok(result)

    }
    async fn write(self, collection_name: &str, docs: Vec<TornadoInOrOut<'a>>) {
        // Get a handle to a collection in the database.
        let collection = self.db.collection::<Document>(collection_name);

        let docs = vec![
            doc! { "title": "1984", "author": "George Orwell" },
            doc! { "title": "Animal Farm", "author": "George Orwell" },
            doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
        ];

        // Insert some documents into the "mydb.books" collection.
        collection.insert_many(docs, None).await;
    }
}
