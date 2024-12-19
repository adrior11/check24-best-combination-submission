use core::f64;
use std::collections::BTreeSet;

use libs::models::dtos::{BestCombinationDto, BestCombinationSubsetDto};

use super::mapper;

pub fn get_best_combination(
    universe: &BTreeSet<usize>,
    subsets: &[BestCombinationSubsetDto],
    limit: usize,
) -> Vec<BestCombinationDto> {
    let mut results: Vec<BestCombinationDto> = Vec::new();
    let mut current_cover: Vec<usize> = Vec::new();
    enumerate_best_combinations(universe, subsets, limit, &mut results, &mut current_cover);
    results
}

fn enumerate_best_combinations(
    universe: &BTreeSet<usize>,
    subsets: &[BestCombinationSubsetDto],
    limit: usize,
    results: &mut Vec<BestCombinationDto>,
    current_cover: &mut Vec<usize>,
) -> bool {
    let covered: BTreeSet<usize> = current_cover
        .iter()
        .flat_map(|&id| {
            subsets
                .iter()
                .find(|s| s.streaming_package_id == id)
                .unwrap()
                .elements
                .clone()
        })
        .collect();

    // Check if all elements are covered
    if covered == *universe {
        // let mut sorted_cover = current_cover.clone();
        // sorted_cover.sort();
        let result = mapper::map_to_best_combination_dto(current_cover, subsets, universe);
        if !results.contains(&result) {
            results.push(result);
            if results.len() >= limit {
                return true; // Signal to stop further recursion
            }
        }
        return false; // Continue searching if limit not reached
    }

    // Prune branches that are too long
    if current_cover.len() >= subsets.len() {
        return false;
    }

    // Calculate cost-benefit ratio for each subset based on uncovered elements
    let mut ratios: Vec<(usize, f64)> = subsets
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            let uncovered_elements = s.elements.difference(&covered).count();
            if uncovered_elements > 0 {
                let cost = s.monthly_price_cents.unwrap_or(usize::MAX) as f64;
                Some((i, cost / uncovered_elements as f64))
            } else {
                None // skip subsets that don't add coverage
            }
        })
        .collect();

    // Sort subsets based on ascending ratio (lower is better)
    ratios.sort_by(|(_, ratio1), (_, ratio2)| {
        ratio1
            .partial_cmp(ratio2)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut branch_explored = true;

    for (i, _) in ratios.iter() {
        current_cover.push(subsets[*i].streaming_package_id);

        // Recurse and check if it should step
        if enumerate_best_combinations(universe, subsets, limit, results, current_cover) {
            return true;
        };

        current_cover.pop();

        // If it exits here, the branch has been fully explored
        branch_explored = false;

        // If limit is reached, stop
        if results.len() >= limit {
            return true;
        }
    }

    // If the branch is fully explored, save the current cover as the closest achievable
    if branch_explored && !current_cover.is_empty() {
        // let mut sorted_cover = current_cover.clone();
        // sorted_cover.sort();
        let result = mapper::map_to_best_combination_dto(current_cover, subsets, universe);
        if !results.contains(&result) {
            results.push(result);
        }
    }

    false // Continue searching
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CONFIG;
    use libs::{
        constants::{DATABASE_NAME, STREAMING_PACKAGE_COLLECTION_NAME},
        db::{dao::StreamingPackageDao, DocumentDatabaseConnector, MongoClient},
    };

    async fn setup() -> (BTreeSet<usize>, Vec<BestCombinationSubsetDto>) {
        dotenv::dotenv().ok();

        let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
        let package_dao = StreamingPackageDao::new(
            mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME),
        );

        // NOTE: Game 8533 of Bayer Muenchen isn't covered by a single offer.
        // The Coverage will never be able to reach 100% given this case.
        let game_ids = BTreeSet::from([
            52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212,
            214, 219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356,
            5305, 5320, 5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436,
            5440, 5422, 5449, 5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548,
            5557, 5566, 5584, 5573, 5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568,
            8560, 8845,
        ]);
        let subsets = package_dao.preprocess_subsets(&game_ids).await;

        assert!(subsets.is_ok());

        (game_ids, subsets.unwrap())
    }

    #[tokio::test]
    async fn test_get_best_combination_without_limit() {
        let (game_ids, subsets) = setup().await;

        let expected = [BestCombinationDto {
            packages: vec![3, 37],
            combined_monthly_price_cents: 999,
            combined_monthly_price_yearly_subscription_in_cents: 699,
            coverage: 99,
        }];
        let expected_package_ids = [[3, 37]];

        let limit = 1;
        let results = get_best_combination(&game_ids, &subsets, limit);
        let result_game_ids: Vec<Vec<usize>> =
            results.iter().cloned().map(|bc| bc.packages).collect();

        assert!(!results.is_empty());
        assert!(!result_game_ids.is_empty());

        assert_eq!(result_game_ids, expected_package_ids);
        assert_eq!(results, expected);
    }

    #[tokio::test]
    async fn test_get_best_combination_with_limit() {
        let (game_ids, subsets) = setup().await;

        let expected = [
            BestCombinationDto {
                packages: vec![3, 37],
                combined_monthly_price_cents: 999,
                combined_monthly_price_yearly_subscription_in_cents: 699,
                coverage: 99,
            },
            BestCombinationDto {
                packages: vec![3, 38],
                combined_monthly_price_cents: 2499,
                combined_monthly_price_yearly_subscription_in_cents: 1999,
                coverage: 99,
            },
            BestCombinationDto {
                packages: vec![3, 10],
                combined_monthly_price_cents: 3599,
                combined_monthly_price_yearly_subscription_in_cents: 2999,
                coverage: 99,
            },
        ];
        let expected_package_ids = [[3, 37], [3, 38], [3, 10]];

        let limit = 3;
        let results = get_best_combination(&game_ids, &subsets, limit);
        let result_game_ids: Vec<Vec<usize>> =
            results.iter().cloned().map(|bc| bc.packages).collect();

        assert!(!results.is_empty());
        assert!(!result_game_ids.is_empty());

        assert_eq!(result_game_ids, expected_package_ids);
        assert_eq!(results, expected);
    }
}
