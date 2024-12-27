use async_graphql::*;
use libs::models::fetch_types::{FetchOptions, FetchResult, FetchStatus};

use super::service::{self, handle_request};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_best_combination(
        &self,
        ctx: &Context<'_>,
        teams: Option<Vec<String>>,
        tournaments: Option<Vec<String>>,
        opts: FetchOptions,
    ) -> async_graphql::Result<FetchResult> {
        service::handle_request(ctx, teams, tournaments, opts).await
    }
}

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn enqueue_best_combination(
        &self,
        ctx: &Context<'_>,
        teams: Option<Vec<String>>,
        tournaments: Option<Vec<String>>,
        opts: FetchOptions,
    ) -> async_graphql::Result<FetchStatus> {
        let result = handle_request(ctx, teams, tournaments, opts).await?;
        Ok(result.status)
    }
}
