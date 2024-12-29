use async_graphql::*;

use libs::models::fetch_types::{FetchOptions, FetchResult, FetchStatus};

use super::service::{self, handle_request};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_best_combination(
        &self,
        ctx: &Context<'_>,
        input: Vec<String>,
        opts: FetchOptions,
    ) -> async_graphql::Result<FetchResult> {
        service::handle_request(ctx, input, opts).await
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn enqueue_best_combination(
        &self,
        ctx: &Context<'_>,
        input: Vec<String>,
        opts: FetchOptions,
    ) -> async_graphql::Result<FetchStatus> {
        let result = handle_request(ctx, input, opts).await?;
        Ok(result.status)
    }
}
