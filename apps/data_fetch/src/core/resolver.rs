use std::sync::Arc;

use async_graphql::*;

use libs::db::dao::GameDao;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn get_teams(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let game_dao = ctx.data::<Arc<GameDao>>()?;
        let teams = game_dao.get_teams().await?;
        Ok(teams)
    }

    async fn get_tournaments(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<String>> {
        let game_dao = ctx.data::<Arc<GameDao>>()?;
        let tournaments = game_dao.get_tournaments().await?;
        Ok(tournaments)
    }

    async fn get_suggestion(
        &self,
        ctx: &Context<'_>,
        input: String,
    ) -> async_graphql::Result<Option<String>> {
        let game_dao = ctx.data::<Arc<GameDao>>()?;
        let suggestion = game_dao.get_suggestion(input).await?;
        Ok(suggestion)
    }
}
