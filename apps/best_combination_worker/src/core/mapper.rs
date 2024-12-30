// BUG: Map games with no monthly_price_cents should lead to Null combined
use std::collections::{BTreeSet, HashMap};

use libs::models::dtos::{
    BestCombinationDto, BestCombinationElementDto, BestCombinationPackageDto,
    BestCombinationSubsetDto,
};

/// Computes a three-stage coverage value (0, 1, 2) from a slice of u8 values,
/// where each value is either `0` (no coverage) or `1` (coverage).
///
/// **Rules**:
/// - Returns `0` if **all** values in the slice are `0`.
/// - Returns `2` if **all** values in the slice are `1`.
/// - Returns `1` otherwise (i.e., partial coverage).
///
/// # Parameters
/// - `values`: A slice of `u8` values representing coverage indicators (0 or 1).
///
/// # Returns
/// - `u8` representing the three-stage coverage:
///   - `0` = no coverage
///   - `1` = partial coverage
///   - `2` = full coverage
fn compute_three_stage_coverage(values: &[u8]) -> u8 {
    if values.is_empty() {
        return 0; // No coverage
    }

    let all_ones = values.iter().all(|&v| v == 1);
    let all_zeroes = values.iter().all(|&v| v == 0);

    if all_ones {
        2
    } else if all_zeroes {
        0
    } else {
        1
    }
}

/// Builds a map of tournament coverage from a set of `BestCombinationElementDto` entries.
/// Groups elements by `tournament_name` and then computes both `live` and `highlights`
/// coverage using [`compute_three_stage_coverage`].
///
/// # Parameters
/// - `elements`: A `BTreeSet` of `BestCombinationElementDto`, each representing a single
///   game/tournament coverage entry.
///
/// # Returns
/// - A `HashMap<String, (u8, u8)>` where:
///   - The `String` key is the tournament name.
///   - The value is a tuple of `(live_coverage, highlights_coverage)`, each computed
///     as a three-stage coverage value (0, 1, 2).
fn build_coverage_map(elements: &BTreeSet<BestCombinationElementDto>) -> HashMap<String, (u8, u8)> {
    let mut coverage_map = HashMap::new();

    let mut grouped_by_tournament: HashMap<String, Vec<&BestCombinationElementDto>> =
        HashMap::new();

    for element in elements {
        grouped_by_tournament
            .entry(element.tournament_name.clone())
            .or_default()
            .push(element);
    }

    for (tournament_name, group) in grouped_by_tournament {
        let live_values: Vec<u8> = group.iter().map(|e| e.live).collect();
        let highlights_values: Vec<u8> = group.iter().map(|e| e.highlights).collect();

        // Compute 3 Stage coverage: [u8] => 0 | 1 | 2
        let live_coverage = compute_three_stage_coverage(&live_values);
        let highlights_coverage = compute_three_stage_coverage(&highlights_values);

        coverage_map.insert(tournament_name, (live_coverage, highlights_coverage));
    }

    coverage_map
}

/// Constructs a `BestCombinationDto` from the given input parameters.
///
/// This function:
/// 1. Filters out subsets whose IDs are in `current_cover`.
/// 2. Builds coverage maps for each selected subset using [`build_coverage_map`].
/// 3. Accumulates pricing information.
/// 4. Computes the combined coverage percentage over the entire `universe`.
///
/// # Parameters
/// - `current_cover`: A list of `streaming_package_id` values (as `usize`) that are included.
/// - `subsets`: A slice of `BestCombinationSubsetDto` describing each streaming package and
///   its set of elements (coverage entries).
/// - `universe`: A `BTreeSet` of all possible `game_id` values, used for computing overall coverage.
///
/// # Returns
/// - A `BestCombinationDto` struct containing:
///   - A list of `BestCombinationPackageDto` for each chosen package.
///   - The combined monthly price, monthly price for yearly subs.
///   - The overall coverage as a percentage (`combined_coverage`).
pub fn map_to_best_combination_dto(
    current_cover: &[usize],
    subsets: &[BestCombinationSubsetDto],
    universe: &BTreeSet<usize>,
) -> BestCombinationDto {
    let mut packages = Vec::new();
    let mut combined_monthly_price_cents = 0;
    let mut combined_monthly_price_yearly_subscription_in_cents = 0;
    let mut covered: BTreeSet<usize> = BTreeSet::new();
    let mut processed_ids: BTreeSet<usize> = BTreeSet::new();

    for subset in subsets {
        let id = subset.streaming_package_id;
        if current_cover.contains(&id) && !processed_ids.contains(&id) {
            processed_ids.insert(id);

            let coverage_map = build_coverage_map(&subset.elements);

            packages.push(BestCombinationPackageDto {
                id: subset.streaming_package_id,
                name: subset.name.clone(),
                coverage: coverage_map,
                monthly_price_cents: subset.monthly_price_cents,
                monthly_price_yearly_subscription_in_cents: subset
                    .monthly_price_yearly_subscription_in_cents,
            });
            combined_monthly_price_cents += subset.monthly_price_cents.unwrap_or(0);
            combined_monthly_price_yearly_subscription_in_cents +=
                subset.monthly_price_yearly_subscription_in_cents;

            for e in &subset.elements {
                if universe.contains(&e.game_id) {
                    covered.insert(e.game_id);
                }
            }
        }
    }

    let combined_coverage = (covered.len() as f64 / universe.len() as f64 * 100.0).round() as u8;

    packages.sort_by(|package1, package2| package1.id.cmp(&package2.id));

    BestCombinationDto {
        packages,
        combined_monthly_price_cents,
        combined_monthly_price_yearly_subscription_in_cents,
        combined_coverage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_stage_coverage_computation() {
        assert_eq!(compute_three_stage_coverage(&[]), 0);
        assert_eq!(compute_three_stage_coverage(&[0, 0]), 0);
        assert_eq!(compute_three_stage_coverage(&[0, 1]), 1);
        assert_eq!(compute_three_stage_coverage(&[1, 1]), 2);
    }

    #[test]
    fn test_build_coverage_map() {
        let elements = BTreeSet::from([
            BestCombinationElementDto::new(1, "A", 1, 1),
            BestCombinationElementDto::new(2, "A", 1, 1),
            BestCombinationElementDto::new(3, "B", 0, 1),
            BestCombinationElementDto::new(4, "B", 1, 0),
            BestCombinationElementDto::new(5, "C", 0, 0),
        ]);

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), (2, 2));
        expected.insert("B".to_string(), (1, 1));
        expected.insert("C".to_string(), (0, 0));

        let coverage_map = build_coverage_map(&elements);

        assert_eq!(coverage_map, expected);
    }

    #[test]
    fn test_mapper() {
        let current_cover = [1, 2, 4];
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "A", 1, 1)]),
                Some(10),
                10,
            ),
            BestCombinationSubsetDto::new(
                2,
                "S2",
                BTreeSet::from([
                    BestCombinationElementDto::new(1, "A", 1, 1),
                    BestCombinationElementDto::new(3, "A", 1, 0),
                ]),
                None,
                10,
            ),
        ];
        let universe = BTreeSet::from([1, 2, 3]);

        let result = map_to_best_combination_dto(&current_cover, &subsets, &universe);
        let expected = BestCombinationDto {
            packages: vec![
                BestCombinationPackageDto::new(1, "S1", vec![("A", (2, 2))], Some(10), 10),
                BestCombinationPackageDto::new(2, "S2", vec![("A", (2, 1))], None, 10),
            ],
            combined_monthly_price_cents: 10,
            combined_monthly_price_yearly_subscription_in_cents: 20,
            combined_coverage: 67,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mapper_with_duplicate_ids() {
        let current_cover = [1];
        let subsets = vec![
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "A", 1, 1)]),
                Some(10),
                10,
            ),
            BestCombinationSubsetDto::new(
                1,
                "S1",
                BTreeSet::from([BestCombinationElementDto::new(1, "A", 1, 1)]),
                Some(10),
                10,
            ),
        ];
        let universe = BTreeSet::from([1]);

        let result = map_to_best_combination_dto(&current_cover, &subsets, &universe);

        // Duplicate packages do not increase the number of packages or sums.
        let expected = BestCombinationDto {
            packages: vec![BestCombinationPackageDto::new(
                1,
                "S1",
                vec![("A", (2, 2))],
                Some(10),
                10,
            )],
            combined_monthly_price_cents: 10,
            combined_monthly_price_yearly_subscription_in_cents: 10,
            combined_coverage: 100,
        };

        assert_eq!(
            result, expected,
            "Mapper should handle duplicate package IDs correctly"
        );
    }
}
