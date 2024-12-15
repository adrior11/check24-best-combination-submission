use mongodb::Collection;
use serde::{de::DeserializeOwned, Serialize};

/// A trait defining the necessary methods for interacting with a document-oriented database.
///
/// This trait abstracts the operations required to initialize a connection to the database
/// and retrieve collections for performing CRUD operations. By implementing this trait,
/// different database clients can provide their specific behaviors while adhering to a common interface.
#[async_trait::async_trait]
pub trait DocumentDatabaseConnector {
    /// Initializes the database connector with the given URI and database name.
    ///
    /// # Parameters
    ///
    /// - `uri`: A `String` representing the connection string to the database.
    /// - `name`: A `String` specifying the name of the database to connect to.
    ///
    /// # Returns
    ///
    /// An instance of the type implementing the `DocumentDatabaseConnector` trait.
    async fn init(uri: &str, name: &str) -> Self;

    /// Retrieves a typed collection from the database.
    ///
    /// This method provides a way to access a specific collection within the database,
    /// allowing operations like querying, inserting, updating, and deleting documents.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type representing the documents within the collection. Must implement
    ///   `Serialize`, `DeserializeOwned`, `Unpin`, `Send`, and `Sync` traits.
    ///
    /// # Parameters
    ///
    /// - `collection_name`: The name of the collection to retrieve.
    ///
    /// # Returns
    ///
    /// A `Collection<T>` instance representing the specified collection.
    fn get_collection<T>(&self, collection_name: &str) -> Collection<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync;
}
