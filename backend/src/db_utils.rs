use crate::model;
use crate::populate_from_csv;

pub(crate) async fn populate_db(client: &mongodb::Client) -> Result<(), Box<dyn std::error::Error>> {
    let database = client.database("best_combination");

    // populate_from_csv!(database, "games", model::Game, "data/bc_game.csv");
    populate_from_csv!(database, "streaming_packages", model::StreamingPackage, "data/bc_streaming_package.csv");
    // populate_from_csv!(database, "streaming_offers", model::StreamingOffer, "data/bc_streaming_offer.csv");

    Ok(())
}
