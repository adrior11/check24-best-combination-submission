use super::DocumentDatabaseConnector;
use mongodb::{bson, Client, Collection};
use serde::{de::DeserializeOwned, Serialize};

/// A MongoDB client implementation of the `DocumentDatabaseConnector` trait.
///
/// This struct encapsulates the MongoDB `Client` and the name of the database it connects to.
/// It provides methods to initialize the connection and retrieve specific collections for operations.
#[derive(Debug, Clone)]
pub struct MongoClient {
    /// The MongoDB client used to interact with the database.
    pub client: Client,

    /// the name of database to connect to.
    pub db_name: String,
}

#[async_trait::async_trait]
impl DocumentDatabaseConnector for MongoClient {
    /// Initializes the `MongoClient` with the provided URI and database name.
    ///
    /// This method establishes a connection to the MongoDB server using the provided URI,
    /// verifies the connection by running a simple `ping` command, and returns an instance
    /// of `MongoClient`.
    ///
    /// # Parameters
    ///
    /// - `uri`: A `String` representing the MongoDB connection string.
    /// - `db_name`: A `String` specifying the name of the database to use.
    ///
    /// # Returns
    ///
    /// An instance of `MongoClient` if the connection and ping are successful.
    ///
    /// # Panics
    ///
    /// This method will panic if:
    ///
    /// - The MongoDB client fails to initialize with the provided URI.
    /// - The `ping` command fails, indicating that the MongoDB server is unreachable.
    ///
    async fn init(uri: &str, db_name: &str) -> Self {
        let client = Client::with_uri_str(uri)
            .await
            .expect("Failed to connect to MongoDB");

        client
            .database("admin")
            .run_command(bson::doc! { "ping": 1 })
            .await
            .expect("Failed to reach MongoDB server");

        MongoClient {
            client,
            db_name: db_name.to_string(),
        }
    }

    fn get_collection<T>(&self, collection_name: &str) -> Collection<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        self.client
            .database(&self.db_name)
            .collection::<T>(collection_name)
    }
}
