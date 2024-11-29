use crate::{api::types::StreamingPackageGQL, app::AppState, core::best_combination_service};
use actix_web::web::Data;
use async_graphql::{Context, Object};
use std::sync::Arc;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn best_combination(
        &self,
        ctx: &Context<'_>,
        teams: Vec<String>,
    ) -> async_graphql::Result<Vec<StreamingPackageGQL>> {
        let app_state = ctx.data::<Arc<AppState>>()?;

        let result =
            best_combination_service::find_best_combination(Data::new(app_state.clone()), teams)
                .await?;

        Ok(result.into_iter().map(StreamingPackageGQL::from).collect())
    }
}
