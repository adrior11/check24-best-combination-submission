use super::{team_mapper, team_service};
use crate::app;
use actix_web::{http::header, web};
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::error;
use std::sync;

#[actix_web::get("/teams")]
async fn get_all_team_names(
    app_state: web::Data<sync::Arc<app::AppState>>,
) -> impl actix_web::Responder {
    let cursor_result = team_service::fetch_all_team_names(app_state).await;

    if let Err(err) = cursor_result {
        log::error!("Error fetching team names: {:?}", err);
        return actix_web::HttpResponse::BadGateway()
            .content_type(header::ContentType::plaintext())
            .body("Database connection issue");
    }

    let team_names: Result<Vec<String>, error::Error> = cursor_result
        .unwrap()
        .filter_map(team_mapper::map_document_to_team_name)
        .try_collect()
        .await;

    match team_names {
        Ok(names) => actix_web::HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .json(names),
        Err(err) => {
            log::error!("Error processing team names: {:?}", err);
            actix_web::HttpResponse::InternalServerError()
                .content_type(header::ContentType::plaintext())
                .body("Error processing team names")
        }
    }
}
