use core::f64;
use std::collections::BTreeSet;

use libs::models::dtos::{BestCombinationDto, BestCombinationSubsetDto};

use super::mapper;
use crate::CONFIG;

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
/// 2. It selects the next best candidate according to the cost per uncovered games.
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
                .iter()
                .map(|elem| elem.game_id)
        })
        .collect();

    // Check if all elements are covered or if a leaf node has been reached
    if covered == *universe || current_cover.len() >= subsets.len() {
        let result =
            mapper::map_to_best_combination_dto(current_cover, subsets, universe, results.len());
        if !results.iter().any(|r| r.is_duplicate_of(&result)) {
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
            let uncovered_elements = s.element_ids().difference(&covered).count();

            if uncovered_elements > 0 {
                let cost = if CONFIG.use_yearly_price {
                    s.monthly_price_yearly_subscription_in_cents as f64
                } else {
                    // Use a high value if monthly_price_cents is None to effectively exclude this subset
                    s.monthly_price_cents.unwrap_or(usize::MAX) as f64
                };
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
        let result =
            mapper::map_to_best_combination_dto(current_cover, subsets, universe, results.len());
        if !results.iter().any(|r| r.is_duplicate_of(&result)) {
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
        models::dtos::{BestCombinationElementDto, BestCombinationPackageDto},
    };

    async fn setup_data() -> (Vec<usize>, Vec<BestCombinationSubsetDto>) {
        dotenv::dotenv().ok();

        let mongo_client = MongoClient::init(&CONFIG.mongodb_uri, DATABASE_NAME).await;
        let package_dao = StreamingPackageDao::new(
            mongo_client.get_collection(STREAMING_PACKAGE_COLLECTION_NAME),
        );

        // Game 8533 of Bayern Muenchen isn't covered by a single offer.
        // The Coverage will never be able to reach 100% given this case.
        let game_ids = vec![
            52, 69, 76, 79, 103, 89, 113, 121, 125, 139, 146, 151, 161, 171, 186, 193, 196, 212,
            214, 219, 225, 240, 251, 257, 261, 272, 284, 293, 307, 320, 302, 325, 337, 349, 356,
            5305, 5320, 5325, 5330, 5341, 5349, 5364, 5367, 5383, 5386, 5394, 5404, 5416, 5436,
            5440, 5422, 5449, 5459, 5467, 5474, 5483, 5492, 5501, 5511, 5525, 5529, 5541, 5548,
            5557, 5566, 5584, 5573, 5593, 7354, 7890, 8440, 8466, 8486, 8514, 8503, 8533, 8568,
            8560, 8845,
        ];
        let subsets = package_dao.aggregate_subsets_by_game_ids(&game_ids).await;

        assert!(subsets.is_ok());

        (game_ids, subsets.unwrap())
    }

    #[test]
    fn test_no_subsets() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![];
        let limit = 5;

        let expected_cover = vec![BestCombinationDto::new(vec![], 0, 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_empty_universe() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::new();
        let subsets = vec![BestCombinationSubsetDto::new(
            1,
            "S1",
            BTreeSet::from([
                BestCombinationElementDto::new(1, "", 1, 1),
                BestCombinationElementDto::new(2, "", 1, 0),
                BestCombinationElementDto::new(3, "", 1, 0),
            ]),
            Some(10),
            10,
        )];
        let limit = 5;

        let expected_cover = vec![BestCombinationDto::new(vec![], 0, 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_empty_universe_no_subsets() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::new();
        let subsets = vec![];
        let limit = 2;

        let expected_cover = vec![BestCombinationDto::new(vec![], 0, 0, 0, 0)];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_configuration_flag() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1]);
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "A", 1, 1)]),
                Some(10),
                100,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([BestCombinationElementDto::new(1, "A", 1, 1)]),
                Some(100),
                10,
            ),
        ];
        let limit = 1;

        let expected_cover = vec![BestCombinationDto::new(
            vec![BestCombinationPackageDto::new(
                1,
                "S1",
                vec![("A", (2, 2))],
                Some(10),
                100,
            )],
            10,
            100,
            100,
            0,
        )];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_single_full_cover() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![BestCombinationSubsetDto::new(
            1,
            "S1",
            BTreeSet::from([
                BestCombinationElementDto::new(1, "A", 1, 1),
                BestCombinationElementDto::new(2, "B", 1, 0),
                BestCombinationElementDto::new(3, "C", 0, 0),
            ]),
            Some(10),
            10,
        )];
        let limit = 5;

        let expected_cover = vec![BestCombinationDto::new(
            vec![BestCombinationPackageDto::new(
                1,
                "S1",
                vec![("A", (2, 2)), ("B", (2, 0)), ("C", (0, 0))],
                Some(10),
                10,
            )],
            10,
            10,
            100,
            0,
        )];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_impossible_coverage() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "", 1, 1)]),
                Some(5),
                10,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([BestCombinationElementDto::new(2, "", 0, 0)]),
                Some(5),
                10,
            ),
            // Element 3 is never covered
        ];
        let limit = 1;

        let expected_cover = vec![BestCombinationDto::new(
            vec![
                BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 10),
                BestCombinationPackageDto::new(2, "S2", vec![("", (0, 0))], Some(5), 10),
            ],
            10,
            20,
            67,
            0,
        )];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(
            results, expected_cover,
            "Should find the next best coverage approximation"
        );
    }

    #[test]
    fn test_duplicate_subsets() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1]);
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "", 1, 1)]),
                Some(5),
                10,
            ),
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "", 1, 1)]),
                Some(5),
                10,
            ),
        ];
        let limit = 2;

        let expected_cover = vec![BestCombinationDto::new(
            vec![BestCombinationPackageDto::new(
                1,
                "S1",
                vec![("", (2, 2))],
                Some(5),
                10,
            )],
            5,
            10,
            100,
            0,
        )];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert!(results.len() == 1);
        assert_eq!(
            results, expected_cover,
            "Should account for duplicate package ids"
        );
    }

    #[test]
    fn test_identical_subsets() {
        dotenv::dotenv().ok();
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "", 1, 1)]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([BestCombinationElementDto::new(1, "", 1, 1)]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                3,
                "S3",
                BTreeSet::from([BestCombinationElementDto::new(2, "", 1, 1)]),
                Some(5),
                5,
            ),
        ];
        let limit = 5;

        // Covers {1,3} and {2,3} as subsets 1 and 2 are identical in coverage and cost,
        // the algorithm should produce distinct solutions since they have different IDs.
        let expected_cover = &[
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                ],
                10,
                10,
                100,
                0,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                ],
                10,
                10,
                100,
                1,
            ),
        ];
        let results = get_best_combinations(&universe, &subsets, limit);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_large_universe() {
        dotenv::dotenv().ok();
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([
                    BestCombinationElementDto::new(1, "", 1, 1),
                    BestCombinationElementDto::new(2, "", 1, 1),
                    BestCombinationElementDto::new(3, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([
                    BestCombinationElementDto::new(2, "", 1, 1),
                    BestCombinationElementDto::new(4, "", 1, 1),
                    BestCombinationElementDto::new(5, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                3,
                "S3",
                BTreeSet::from([
                    BestCombinationElementDto::new(3, "", 1, 1),
                    BestCombinationElementDto::new(6, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                4,
                "S4",
                BTreeSet::from([
                    BestCombinationElementDto::new(7, "", 1, 1),
                    BestCombinationElementDto::new(8, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                5,
                "S5",
                BTreeSet::from([
                    BestCombinationElementDto::new(9, "", 1, 1),
                    BestCombinationElementDto::new(10, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                6,
                "S6",
                BTreeSet::from([
                    BestCombinationElementDto::new(4, "", 1, 1),
                    BestCombinationElementDto::new(7, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                7,
                "S7",
                BTreeSet::from([
                    BestCombinationElementDto::new(5, "", 1, 1),
                    BestCombinationElementDto::new(8, "", 1, 1),
                    BestCombinationElementDto::new(9, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                8,
                "S8",
                BTreeSet::from([
                    BestCombinationElementDto::new(10, "", 1, 1),
                    BestCombinationElementDto::new(1, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
        ];
        let limit = 5;

        let expected_cover = &[
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                25,
                25,
                100,
                0,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(8, "S8", vec![("", (2, 2))], Some(5), 5),
                ],
                25,
                25,
                100,
                1,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(4, "S4", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                30,
                30,
                100,
                2,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(4, "S4", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(8, "S8", vec![("", (2, 2))], Some(5), 5),
                ],
                30,
                30,
                100,
                3,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(3, "S3", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                30,
                30,
                100,
                4,
            ),
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
        dotenv::dotenv().ok();
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([
                    BestCombinationElementDto::new(1, "", 1, 1),
                    BestCombinationElementDto::new(2, "", 1, 1),
                    BestCombinationElementDto::new(3, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([
                    BestCombinationElementDto::new(2, "", 1, 1),
                    BestCombinationElementDto::new(4, "", 1, 1),
                    BestCombinationElementDto::new(5, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                4,
                "S4",
                BTreeSet::from([
                    BestCombinationElementDto::new(7, "", 1, 1),
                    BestCombinationElementDto::new(8, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                5,
                "S5",
                BTreeSet::from([
                    BestCombinationElementDto::new(9, "", 1, 1),
                    BestCombinationElementDto::new(10, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                6,
                "S6",
                BTreeSet::from([
                    BestCombinationElementDto::new(4, "", 1, 1),
                    BestCombinationElementDto::new(7, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                7,
                "S7",
                BTreeSet::from([
                    BestCombinationElementDto::new(5, "", 1, 1),
                    BestCombinationElementDto::new(8, "", 1, 1),
                    BestCombinationElementDto::new(9, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
            BestCombinationSubsetDto::new(
                8,
                "S8",
                BTreeSet::from([
                    BestCombinationElementDto::new(10, "", 1, 1),
                    BestCombinationElementDto::new(1, "", 1, 1),
                ]),
                Some(5),
                5,
            ),
        ];
        let limit = 5;

        // Element 6 of the universe is never being covered, as S3 is missing.
        let expected_cover = &[
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                20,
                20,
                90,
                0,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(8, "S8", vec![("", (2, 2))], Some(5), 5),
                ],
                20,
                20,
                90,
                1,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(4, "S4", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                25,
                25,
                90,
                2,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(4, "S4", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(8, "S8", vec![("", (2, 2))], Some(5), 5),
                ],
                25,
                25,
                90,
                3,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(1, "S1", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(2, "S2", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(5, "S5", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(6, "S6", vec![("", (2, 2))], Some(5), 5),
                    BestCombinationPackageDto::new(7, "S7", vec![("", (2, 2))], Some(5), 5),
                ],
                25,
                25,
                90,
                4,
            ),
        ];
        let results = get_best_combinations(&universe, &subsets, limit);
        dbg!(&results);
        dbg!(&expected_cover);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);
    }

    #[tokio::test]
    async fn test_get_best_combination_without_limit() {
        dotenv::dotenv().ok();
        let (game_ids, subsets) = setup_data().await;

        let expected = [BestCombinationDto::new(
            vec![
                BestCombinationPackageDto::new(
                    3,
                    "ZDF - Free-TV",
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("Bundesliga 24/25", (0, 2)),
                        ("Bundesliga 23/24", (0, 2)),
                    ],
                    Some(0),
                    0,
                ),
                BestCombinationPackageDto::new(
                    37,
                    "DAZN - World",
                    vec![
                        ("UEFA Champions League 24/25", (0, 2)),
                        ("DFB Pokal 24/25", (0, 2)),
                    ],
                    Some(999),
                    699,
                ),
            ],
            999,
            699,
            99,
            0,
        )];

        let limit = 1;
        let universe: BTreeSet<usize> = game_ids.iter().copied().collect();
        let results = get_best_combinations(&universe, &subsets, limit);

        assert!(!results.is_empty());
        assert_eq!(results, expected);
    }

    #[tokio::test]
    async fn test_get_best_combination_with_limit() {
        dotenv::dotenv().ok();
        let (game_ids, subsets) = setup_data().await;

        let expected = [
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(
                        3,
                        "ZDF - Free-TV",
                        vec![
                            ("UEFA Champions League 24/25", (0, 2)),
                            ("Bundesliga 24/25", (0, 2)),
                            ("Bundesliga 23/24", (0, 2)),
                        ],
                        Some(0),
                        0,
                    ),
                    BestCombinationPackageDto::new(
                        37,
                        "DAZN - World",
                        vec![
                            ("UEFA Champions League 24/25", (0, 2)),
                            ("DFB Pokal 24/25", (0, 2)),
                        ],
                        Some(999),
                        699,
                    ),
                ],
                999,
                699,
                99,
                0,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(
                        3,
                        "ZDF - Free-TV",
                        vec![
                            ("UEFA Champions League 24/25", (0, 2)),
                            ("Bundesliga 24/25", (0, 2)),
                            ("Bundesliga 23/24", (0, 2)),
                        ],
                        Some(0),
                        0,
                    ),
                    BestCombinationPackageDto::new(
                        38,
                        "DAZN - Super Sports",
                        vec![
                            ("UEFA Champions League 24/25", (0, 2)),
                            ("DFB Pokal 24/25", (0, 2)),
                        ],
                        Some(2499),
                        1999,
                    ),
                ],
                2499,
                1999,
                99,
                1,
            ),
            BestCombinationDto::new(
                vec![
                    BestCombinationPackageDto::new(
                        3,
                        "ZDF - Free-TV",
                        vec![
                            ("UEFA Champions League 24/25", (0, 2)),
                            ("Bundesliga 24/25", (0, 2)),
                            ("Bundesliga 23/24", (0, 2)),
                        ],
                        Some(0),
                        0,
                    ),
                    BestCombinationPackageDto::new(
                        10,
                        "WOW - Live-Sport",
                        vec![("Bundesliga 24/25", (1, 2)), ("DFB Pokal 24/25", (2, 2))],
                        Some(3599),
                        2999,
                    ),
                ],
                3599,
                2999,
                99,
                2,
            ),
        ];

        let limit = 3;
        let universe: BTreeSet<usize> = game_ids.iter().copied().collect();
        let results = get_best_combinations(&universe, &subsets, limit);

        assert!(!results.is_empty());
        assert_eq!(results, expected);
    }
}
