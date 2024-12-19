use std::{collections::BTreeSet, env};

use libs::{
    constants::{
        DATABASE_NAME, GAME_COLLECTION_NAME, STREAMING_OFFER_COLLECTION_NAME,
        STREAMING_PACKAGE_COLLECTION_NAME,
    },
    db::{
        dao::{GameDao, StreamingOfferDao, StreamingPackageDao},
        DocumentDatabaseConnector, MongoClient,
    },
};

#[tokio::test]
async fn test_int_fetch_games() {
    dotenv::dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;

    let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

    let teams = vec!["Bayern München".to_string()];
    let games = game_dao.fetch_games(&teams).await.unwrap();

    assert!(!games.is_empty(), "No games fetched");
    for game in games {
        assert!(
            teams.contains(&game.team_home) || teams.contains(&game.team_away),
            "Fetched game does not involve the specified teams"
        );
    }
}

#[tokio::test]
async fn test_int_fetch_game_ids() {
    dotenv::dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;

    let game_dao = GameDao::new(mongo_client.get_collection(GAME_COLLECTION_NAME));

    let teams = vec!["Bayern München".to_string()];
    let game_ids = game_dao.fetch_game_ids(&teams).await.unwrap();

    assert!(!game_ids.is_empty(), "No games fetched");
}

#[tokio::test]
async fn test_int_fetch_offers() {
    dotenv::dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;

    let offer_dao =
        StreamingOfferDao::new(mongo_client.get_collection(STREAMING_OFFER_COLLECTION_NAME));

    let game_ids = vec![
        52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212, 214,
        219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356, 5305, 5320,
        5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436, 5440, 5422, 5449,
        5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548, 5557, 5566, 5584, 5573,
        5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568, 8560, 8845,
    ];
    let offers = offer_dao.fetch_offers(&game_ids).await.unwrap();

    assert!(!offers.is_empty(), "No offers fetched");
    for offer in &offers {
        assert!(
            game_ids.contains(&offer.game_id),
            "Fetched offer does not correspond to the specified game_ids"
        );
    }
}

#[tokio::test]
async fn test_int_fetch_packages() {
    dotenv::dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;

    let package_dao =
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME));

    let package_ids = vec![
        37, 55, 14, 10, 38, 17, 13, 19, 15, 2, 56, 54, 43, 18, 20, 50, 47, 35, 4, 41, 39, 53, 52,
        16, 44, 49, 3, 36, 40,
    ];
    let packages = package_dao
        .fetch_packages(&package_ids)
        .await
        .expect("Failed to fetch packages");

    assert!(!packages.is_empty(), "No packages fetched");
    for package in packages {
        assert!(
            package_ids.contains(&package.streaming_package_id),
            "Fetched package does not correspond to the specified package_ids"
        );
    }
}

#[tokio::test]
async fn test_int_preprocess_subsets() {
    dotenv::dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set in env");
    let mongo_client = MongoClient::init(&uri, DATABASE_NAME).await;
    let package_dao =
        StreamingPackageDao::new(mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME));

    let game_ids = BTreeSet::from([
        52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212, 214,
        219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356, 5305, 5320,
        5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436, 5440, 5422, 5449,
        5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548, 5557, 5566, 5584, 5573,
        5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568, 8560, 8845,
    ]);
    let subsets_result = package_dao.preprocess_subsets(&game_ids).await;
    assert!(subsets_result.is_ok());

    let subsets = subsets_result.unwrap();
    assert!(!subsets.is_empty());

    for subset in &subsets {
        let intersection: Vec<_> = subset.elements.intersection(&game_ids).cloned().collect();
        assert!(!intersection.is_empty());
    }
}
