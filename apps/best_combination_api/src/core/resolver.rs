use std::sync::Arc;

use async_graphql::*;

use libs::{
    caching::{self, CacheValue, CompositeKey, RedisClient},
    db::dao::GameDao,
    messaging::{self, MqChannel},
    models::{
        dtos::BestCombinationDto,
        fetch_types::{FetchOptions, FetchResult, FetchStatus},
        payloads::TaskMessagePayload,
    },
};

use crate::CONFIG;

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

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
        let game_dao = ctx.data::<Arc<GameDao>>()?;
        let redis_client = ctx.data::<Arc<RedisClient>>()?;
        let mq_channel = ctx.data::<Arc<MqChannel>>()?;

        let game_ids = game_dao
            .aggregate_game_ids(teams.clone(), tournaments)
            .await?;
        if game_ids.is_empty() {
            return Err(Error::new(format!(
                "Unknown input: no matching games found for teams {:?}",
                teams
            )));
        }

        let key = CompositeKey::new(game_ids.clone(), opts.clone());

        if let Some(cached_entry) = caching::get_cached_entry(redis_client, &key).await? {
            match cached_entry.value {
                CacheValue::Processing => {
                    return Ok(FetchResult {
                        status: FetchStatus::Processing,
                        ids: game_ids,
                        data: None,
                    })
                }
                CacheValue::Data(data) => {
                    return Ok(FetchResult {
                        status: FetchStatus::Ready,
                        ids: game_ids,
                        data: Some(data),
                    })
                }
            }
        }

        caching::cache_entry(
            redis_client,
            &key,
            CacheValue::<Vec<BestCombinationDto>>::Processing,
        )
        .await?;

        let payload = TaskMessagePayload::from(key);
        let job_enqueued = messaging::enqueue_job(mq_channel, &CONFIG.task_queue_name, &payload)
            .await
            .is_ok();

        let status = if job_enqueued {
            FetchStatus::Processing
        } else {
            FetchStatus::Error
        };

        Ok(FetchResult {
            status,
            ids: game_ids,
            data: None,
        })
    }
}

// TODO: Mutation
