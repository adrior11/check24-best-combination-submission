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

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::{bson, error};

    #[tokio::test]
    async fn test_map_document_to_team_name_valid() {
        let valid_document = Ok(bson::doc! { "team_name": "Team A" });
        let result = map_document_to_team_name(valid_document).await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), "Team A");
    }

    #[tokio::test]
    async fn test_map_document_to_team_name_missing_field() {
        let invalid_document = Ok(bson::doc! { "other_field": "Not a team" });
        let result = map_document_to_team_name(invalid_document).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_map_document_to_team_name_error_document() {
        let mock_error = error::Error::custom("Invalid document");
        let error_document: Result<bson::Document, error::Error> = Err(mock_error);
        let result = map_document_to_team_name(error_document).await;

        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }
}
