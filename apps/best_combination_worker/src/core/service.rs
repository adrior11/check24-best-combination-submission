use core::f64;
use std::collections::BTreeSet;

use libs::models::dtos::{BestCombinationDto, BestCombinationSubsetDto};

use super::mapper;

/// Computes a set of best combinations of streaming package subsets that cover a given universe of game IDs.
///
/// # Overview
///
/// `get_best_combinations` attempts to find one or more best combinations of these packages (subsets)
/// that cover the entire universe of game IDs. A combination is considered covering the universe if
/// every game ID in the universe is included in at least one offer of a chosen package.
///
/// In addition, it can also consider use cases with non-existent set coverage, where it tries to
/// approximate an arbitrary number of solutions, which get as close as possible.
///
/// Under the hood, this method uses a greedy recursive backtracking strategy, guided by heuristics like
/// sorting subsets according to cost or cost-per-uncovered-element ratios. While heuristics and pruning
/// strategies may help in practice, the underlying problem is NP-hard. Thus, this algorithm can still
/// exhibit exponential runtime in the worst case.
///
/// # Example Scenario
///
/// Suppose we have a universe U = {1, 2} and subsets:
/// - S1 covers {1} with cost 5
/// - S2 covers {1} with cost 5
/// - S3 covers {2} with cost 5
///
/// Both (S1, S3) and (S2, S3) form covers of U, making multiple equally viable solutions.
/// The algorithm enumerates these solutions, which can be beneficial if you want a set
/// of candidate solutions for further analysis.
///
/// # Arguments
///
/// * `universe` - A `BTreeSet<usize>` representing all game IDs that must be covered.
/// * `subsets` - A slice of `BestCombinationSubsetDto` representing candidate streaming packages.
/// * `limit` - The maximum number of solutions (combinations of subsets) to return.
///
/// # Returns
///
/// `Vec<BestCombinationDto>`: A vector of best combinations.
///
pub fn get_best_combinations(
    universe: &BTreeSet<usize>,
    subsets: &[BestCombinationSubsetDto],
    limit: usize,
) -> Vec<BestCombinationDto> {
    let mut results: Vec<BestCombinationDto> = Vec::new();
    let mut current_cover: Vec<usize> = Vec::new();
    enumerate_best_combinations(universe, subsets, limit, &mut results, &mut current_cover);
    results
}

/// Recursively enumerates possible combinations of subsets that cover the given universe of game IDs.
///
/// # Overview
///
/// `enumerate_best_combinations` is the core logic behind `get_best_combinations`. Using backtracking,
/// it attempts to build complete solutions by selecting subsets:
///
/// 1. At each recursive call, it evaluates which subsets best improve coverage of the remaining uncovered games.
/// 2. It selects the next best candidate according to the cost per uncovered games
/// 3. If a full cover is found or it reaches a leaf node, the current combination is recorded as a solution.
/// 4. The function then attempts to find more solutions (up to the specified `limit`) by backtracking and trying
///    alternate subsets.
///
/// # Arguments
///
/// * `universe` - The full set of game IDs that must be covered.
/// * `subsets` - The collection of candidate streaming packages (no duplicates assumed).
/// * `limit` - The maximum number of solutions to return. Once reached, the search halts.
/// * `results` - A mutable reference to a vector collecting all found solutions.
/// * `current_cover` - A mutable vector representing the current partial solution (as a list of chosen subset IDs).
///
/// # Returns
///
/// Returns `true` if more solutions can still be found (meaning it will continue searching), or `false`
/// if the limit has been reached or no further solutions are possible.
///
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

    // Check if all elements are covered or if a leaf node has been reached
    if covered == *universe || current_cover.len() >= subsets.len() {
        let result = mapper::map_to_best_combination_dto(current_cover, subsets, universe);
        if !results.contains(&result) {
            results.push(result);
            if results.len() >= limit {
                return true; // Signal to stop further recursion
            }
        }
        return false; // Continue searching if limit not reached
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

    fn create_best_combination_dto(
        packages: Vec<usize>,
        combined_monthly_price_cents: usize,
        combined_monthly_price_yearly_subscription_in_cents: usize,
        coverage: u8,
    ) -> BestCombinationDto {
        BestCombinationDto {
            packages,
            combined_monthly_price_cents,
            combined_monthly_price_yearly_subscription_in_cents,
            coverage,
        }
    }

    fn create_best_combination_subset_dto(
        streaming_package_id: usize,
        elements: BTreeSet<usize>,
        monthly_price_cents: Option<usize>,
        monthly_price_yearly_subscription_in_cents: usize,
    ) -> BestCombinationSubsetDto {
        BestCombinationSubsetDto {
            streaming_package_id,
            elements,
            monthly_price_cents,
            monthly_price_yearly_subscription_in_cents,
        }
    }

    async fn setup_data() -> (BTreeSet<usize>, Vec<BestCombinationSubsetDto>) {
        dotenv::dotenv().ok();

        let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
        let package_dao = StreamingPackageDao::new(
            mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME),
        );

        // Game 8533 of Bayer Muenchen isn't covered by a single offer.
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

    #[test]
    fn test_no_subsets() {
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![];
        let limit = 5;

        let expected_cover = vec![create_best_combination_dto(vec![], 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_empty_universe() {
        let universe = BTreeSet::new();
        let subsets = vec![create_best_combination_subset_dto(
            1,
            BTreeSet::from([1, 2, 3]),
            Some(10),
            10,
        )];
        let limit = 5;

        let expected_cover = vec![create_best_combination_dto(vec![], 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_empty_universe_no_subsets() {
        let universe = BTreeSet::new();
        let subsets = vec![];
        let limit = 2;

        let expected_cover = vec![create_best_combination_dto(vec![], 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_single_full_cover() {
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![create_best_combination_subset_dto(
            1,
            BTreeSet::from([1, 2, 3]),
            Some(10),
            10,
        )];
        let limit = 5;

        let expected_cover = vec![create_best_combination_dto(vec![1], 10, 10, 100)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_impossible_coverage() {
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![
            create_best_combination_subset_dto(1, BTreeSet::from([1]), Some(5), 10),
            create_best_combination_subset_dto(2, BTreeSet::from([2]), Some(5), 10),
            // Element 3 is never covered
        ];
        let limit = 1;

        let expected_cover = vec![create_best_combination_dto(vec![1, 2], 10, 20, 67)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(
            results, expected_cover,
            "Should find the next best coverage approximation"
        );
    }

    #[test]
    fn test_duplicate_subsets() {
        let universe = BTreeSet::from([1]);
        let subsets = vec![
            create_best_combination_subset_dto(1, BTreeSet::from([1]), Some(10), 10),
            create_best_combination_subset_dto(1, BTreeSet::from([1]), Some(10), 10),
        ];
        let limit = 2;

        let expected_cover = vec![create_best_combination_dto(vec![1], 10, 10, 100)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert!(results.len() == 1);
        assert_eq!(
            results, expected_cover,
            "Should account for duplicate package ids"
        );
    }

    #[test]
    fn test_identical_subsets() {
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![
            create_best_combination_subset_dto(1, BTreeSet::from([1]), Some(5), 5),
            create_best_combination_subset_dto(2, BTreeSet::from([1]), Some(5), 5),
            create_best_combination_subset_dto(3, BTreeSet::from([2]), Some(5), 5),
        ];
        let limit = 5;

        // Covers {1,3} and {2,3} as subsets 1 and 2 are identical in coverage and cost,
        // the algorithm should produce distinct solutions since they have different IDs.
        let expected_cover = &[
            create_best_combination_dto(vec![1, 3], 10, 10, 100),
            create_best_combination_dto(vec![2, 3], 10, 10, 100),
        ];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_large_universe() {
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            create_best_combination_subset_dto(1, BTreeSet::from([1, 2, 3]), Some(10), 10),
            create_best_combination_subset_dto(2, BTreeSet::from([2, 4, 5]), Some(10), 10),
            create_best_combination_subset_dto(3, BTreeSet::from([3, 6]), Some(10), 10),
            create_best_combination_subset_dto(4, BTreeSet::from([7, 8]), Some(10), 10),
            create_best_combination_subset_dto(5, BTreeSet::from([9, 10]), Some(10), 10),
            create_best_combination_subset_dto(6, BTreeSet::from([4, 7]), Some(10), 10),
            create_best_combination_subset_dto(7, BTreeSet::from([5, 8, 9]), Some(10), 10),
            create_best_combination_subset_dto(8, BTreeSet::from([10, 1]), Some(10), 10),
        ];
        let limit = 5;

        let expected_cover = &[
            create_best_combination_dto(vec![1, 3, 5, 6, 7], 50, 50, 100),
            create_best_combination_dto(vec![1, 3, 6, 7, 8], 50, 50, 100),
            create_best_combination_dto(vec![1, 2, 3, 4, 5, 7], 60, 60, 100),
            create_best_combination_dto(vec![1, 2, 3, 4, 7, 8], 60, 60, 100),
            create_best_combination_dto(vec![1, 2, 3, 5, 6, 7], 60, 60, 100),
        ];
        let results = get_best_combinations(&universe, &subsets, limit);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_uncoverable_approximation() {
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            create_best_combination_subset_dto(1, BTreeSet::from([1, 2, 3]), Some(10), 10),
            create_best_combination_subset_dto(2, BTreeSet::from([2, 4, 5]), Some(10), 10),
            create_best_combination_subset_dto(4, BTreeSet::from([7, 8]), Some(10), 10),
            create_best_combination_subset_dto(5, BTreeSet::from([9, 10]), Some(10), 10),
            create_best_combination_subset_dto(6, BTreeSet::from([4, 7]), Some(10), 10),
            create_best_combination_subset_dto(7, BTreeSet::from([5, 8, 9]), Some(10), 10),
            create_best_combination_subset_dto(8, BTreeSet::from([10, 1]), Some(10), 10),
        ];
        let limit = 5;

        // Element 6 of the universe is never being covered, as S3 is missing.
        let expected_cover = &[
            create_best_combination_dto(vec![1, 5, 6, 7], 40, 40, 90),
            create_best_combination_dto(vec![1, 6, 7, 8], 40, 40, 90),
            create_best_combination_dto(vec![1, 2, 4, 5, 7], 50, 50, 90),
            create_best_combination_dto(vec![1, 2, 4, 7, 8], 50, 50, 90),
            create_best_combination_dto(vec![1, 2, 5, 6, 7], 50, 50, 90),
        ];
        let results = get_best_combinations(&universe, &subsets, limit);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);
    }

    #[tokio::test]
    async fn test_get_best_combination_without_limit() {
        let (game_ids, subsets) = setup_data().await;

        let expected = [BestCombinationDto {
            packages: vec![3, 37],
            combined_monthly_price_cents: 999,
            combined_monthly_price_yearly_subscription_in_cents: 699,
            coverage: 99,
        }];
        let expected_package_ids = [[3, 37]];

        let limit = 1;
        let results = get_best_combinations(&game_ids, &subsets, limit);
        let result_game_ids: Vec<Vec<usize>> =
            results.iter().cloned().map(|bc| bc.packages).collect();

        assert!(!results.is_empty());
        assert!(!result_game_ids.is_empty());

        assert_eq!(result_game_ids, expected_package_ids);
        assert_eq!(results, expected);
    }

    #[tokio::test]
    async fn test_get_best_combination_with_limit() {
        let (game_ids, subsets) = setup_data().await;

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
        let results = get_best_combinations(&game_ids, &subsets, limit);
        let result_game_ids: Vec<Vec<usize>> =
            results.iter().cloned().map(|bc| bc.packages).collect();

        assert!(!results.is_empty());
        assert!(!result_game_ids.is_empty());

        assert_eq!(result_game_ids, expected_package_ids);
        assert_eq!(results, expected);
    }
}
