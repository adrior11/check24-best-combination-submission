use std::collections::BTreeSet;

use libs::models::payloads::TaskMessagePayload;

use super::types::FetchOptions;

pub fn map_task_message_payload(game_ids: Vec<usize>, opts: FetchOptions) -> TaskMessagePayload {
    TaskMessagePayload {
        game_ids: BTreeSet::from_iter(game_ids),
        limit: opts.limit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper() {
        let game_ids = vec![1, 2, 3];
        let opts = FetchOptions { limit: 3 };

        let result = map_task_message_payload(game_ids, opts);
        let expected = TaskMessagePayload {
            game_ids: BTreeSet::from([1, 2, 3]),
            limit: 3,
        };

        assert_eq!(result, expected);
    }
}
