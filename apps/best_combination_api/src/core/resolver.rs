use std::sync::Arc;

use async_graphql::*;
use libs::{
    caching::{self, CacheValue, RedisClient},
    db::dao::GameDao,
    messaging::{self, MqChannel},
    models::dtos::BestCombinationDto,
};

use super::{
    mapper,
    types::{FetchOptions, FetchResult, FetchStatus},
};
use crate::CONFIG;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_best_combination(
        &self,
        ctx: &Context<'_>,
        teams: Vec<String>,
        opts: FetchOptions,
    ) -> async_graphql::Result<FetchResult> {
        let game_dao = ctx.data::<Arc<GameDao>>()?;
        let redis_client = ctx.data::<Arc<RedisClient>>()?;
        let mq_channel = ctx.data::<Arc<MqChannel>>()?;

        let game_ids = game_dao.fetch_game_ids(&teams).await?; // NOTE: Can data-fetch do this?
        if game_ids.is_empty() {
            return Err(Error::new(format!(
                "Unknown input: no matching games found for teams {:?}",
                teams
            )));
        }

        if let Some(cached_entry) = caching::get_cached_entry(redis_client, &game_ids).await? {
            match cached_entry.value {
                CacheValue::Processing => {
                    return Ok(FetchResult {
                        status: FetchStatus::Processing,
                        data: None,
                    })
                }
                CacheValue::Data(data) => {
                    return Ok(FetchResult {
                        status: FetchStatus::Ready,
                        data: Some(data),
                    })
                }
            }
        }

        caching::cache_entry(
            redis_client,
            game_ids.clone(),
            CacheValue::<Vec<BestCombinationDto>>::Processing,
        )
        .await?;

        let payload = mapper::map_task_message_payload(game_ids, opts);
        let job_enqueued = messaging::enqueue_job(mq_channel, &CONFIG.task_queue_name, &payload)
            .await
            .is_ok();

        let status = if job_enqueued {
            FetchStatus::Processing
        } else {
            FetchStatus::Error
        };

        Ok(FetchResult { status, data: None })
    }
}

// TODO: Mutation
