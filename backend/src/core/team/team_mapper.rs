use mongodb::bson;

pub async fn map_document_to_team_name(
    document: Result<bson::Document, mongodb::error::Error>,
) -> Option<Result<String, mongodb::error::Error>> {
    match document {
        Ok(doc) => match doc.get("team_name") {
            Some(bson::Bson::String(team_name)) => Some(Ok(team_name.to_owned())),
            _ => {
                log::warn!("Document missing valid 'team_name': {:?}", doc);
                None
            }
        },
        Err(err) => {
            log::error!("Error while iterating cursor: {:?}", err);
            Some(Err(err))
        }
    }
}
