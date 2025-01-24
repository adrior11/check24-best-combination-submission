use std::collections::BTreeSet;

use crate::util::Subset;

pub fn recursive_set_covers(
    universe: &BTreeSet<usize>,
    subsets: &[Subset],
    limit: usize,
) -> Vec<Vec<usize>> {
    let mut results: Vec<Vec<usize>> = Vec::new();
    let mut current_cover: Vec<usize> = Vec::new();
    enumerate_recursive_set_cover(universe, subsets, limit, &mut results, &mut current_cover);
    results
}

fn enumerate_recursive_set_cover(
    universe: &BTreeSet<usize>,
    subsets: &[Subset],
    limit: usize,
    results: &mut Vec<Vec<usize>>,
    current_cover: &mut Vec<usize>,
) -> bool {
    let covered: BTreeSet<usize> = current_cover
        .iter()
        .flat_map(|&id| {
            subsets
                .iter()
                .find(|s| s.id == id)
                .unwrap()
                .elements
                .clone()
        })
        .collect();

    // Check if all elements are covered or if a leaf node has been reached
    if covered == *universe || current_cover.len() >= subsets.len() {
        let mut sorted_cover = current_cover.clone();
        sorted_cover.sort();
        if !results.contains(&sorted_cover) {
            results.push(sorted_cover);
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
                Some((i, s.cost as f64 / uncovered_elements as f64))
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
        current_cover.push(subsets[*i].id);

        // Recurse and check if it should step
        if enumerate_recursive_set_cover(universe, subsets, limit, results, current_cover) {
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
        let mut sorted_cover = current_cover.clone();
        sorted_cover.sort();
        if !results.contains(&sorted_cover) {
            results.push(sorted_cover);
        }
    }

    false // Continue searching
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_results(results: &mut [Vec<usize>]) {
        results.iter_mut().for_each(|res| res.sort());
    }

    #[test]
    fn test_no_subsets() {
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![];
        let limit = 5;

        let expected_cover: &[Vec<usize>] = &[vec![]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_empty_universe() {
        let universe = BTreeSet::new();
        let subsets = vec![Subset {
            id: 1,
            elements: BTreeSet::from([1, 2, 3]),
            cost: 10,
        }];
        let limit = 2;

        let expected_cover: &[Vec<usize>] = &[vec![]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover, "Should handle empty universe");
    }

    #[test]
    fn test_empty_universe_no_subsets() {
        let universe = BTreeSet::new();
        let subsets = vec![];
        let limit = 5;

        let expected_cover: &[Vec<usize>] = &[vec![]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_single_full_cover() {
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![Subset {
            id: 1,
            elements: BTreeSet::from([1, 2, 3]),
            cost: 10,
        }];
        let limit = 5;

        let expected_cover = &[vec![1]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_impossible_coverage() {
        let universe = BTreeSet::from([1, 2, 3]);
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1]),
                cost: 5,
            },
            Subset {
                id: 2,
                elements: BTreeSet::from([2]),
                cost: 5,
            },
            // Element 3 is never covered
        ];
        let limit = 3;

        let expected_cover = &[vec![1, 2]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_duplicate_subsets() {
        let universe = BTreeSet::from([1]);
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1]),
                cost: 10,
            },
            Subset {
                id: 1,
                elements: BTreeSet::from([1]),
                cost: 10,
            },
        ];
        let limit = 2;
        let expected_cover = &[[1]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_identical_subsets() {
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1]),
                cost: 5,
            },
            Subset {
                id: 2,
                elements: BTreeSet::from([1]),
                cost: 5,
            },
            Subset {
                id: 3,
                elements: BTreeSet::from([2]),
                cost: 5,
            },
        ];
        let limit = 5;

        // Covers {1,3} and {2,3} as subsets 1 and 2 are identical in coverage and cost,
        // the algorithm should produce distinct solutions since they have different IDs.
        let expected_cover = &[vec![1, 3], vec![2, 3]];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_large_universe() {
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1, 2, 3]),
                cost: 10,
            },
            Subset {
                id: 2,
                elements: BTreeSet::from([2, 4, 5]),
                cost: 10,
            },
            Subset {
                id: 3,
                elements: BTreeSet::from([3, 6]),
                cost: 10,
            },
            Subset {
                id: 4,
                elements: BTreeSet::from([7, 8]),
                cost: 10,
            },
            Subset {
                id: 5,
                elements: BTreeSet::from([9, 10]),
                cost: 10,
            },
            Subset {
                id: 6,
                elements: BTreeSet::from([4, 7]),
                cost: 10,
            },
            Subset {
                id: 7,
                elements: BTreeSet::from([5, 8, 9]),
                cost: 10,
            },
            Subset {
                id: 8,
                elements: BTreeSet::from([10, 1]),
                cost: 10,
            },
        ];
        let limit = 5;

        let expected_cover = &[
            vec![1, 3, 5, 6, 7],
            vec![1, 3, 6, 7, 8],
            vec![1, 2, 3, 4, 5, 7],
            vec![1, 2, 3, 4, 7, 8],
            vec![1, 2, 3, 5, 6, 7],
        ];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);

        for cover in &results {
            let covered: BTreeSet<_> = cover
                .iter()
                .flat_map(|&id| {
                    subsets
                        .iter()
                        .find(|s| s.id == id)
                        .unwrap()
                        .elements
                        .iter()
                        .cloned()
                })
                .collect();
            assert_eq!(
                covered, universe,
                "Every cover must cover the entire universe"
            );
        }

        println!("results: {:?}", results);
    }

    #[test]
    fn test_uncoverable_approximation() {
        let universe: BTreeSet<_> = (1..=10).collect();
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1, 2, 3]),
                cost: 10,
            },
            Subset {
                id: 2,
                elements: BTreeSet::from([2, 4, 5]),
                cost: 10,
            },
            Subset {
                id: 4,
                elements: BTreeSet::from([7, 8]),
                cost: 10,
            },
            Subset {
                id: 5,
                elements: BTreeSet::from([9, 10]),
                cost: 10,
            },
            Subset {
                id: 6,
                elements: BTreeSet::from([4, 7]),
                cost: 10,
            },
            Subset {
                id: 7,
                elements: BTreeSet::from([5, 8, 9]),
                cost: 10,
            },
            Subset {
                id: 8,
                elements: BTreeSet::from([10, 1]),
                cost: 10,
            },
        ];
        let limit = 5;

        // Element 6 of the universe is never being covered, as S3 is missing.
        let expected_cover = &[
            vec![1, 5, 6, 7],
            vec![1, 6, 7, 8],
            vec![1, 2, 4, 5, 7],
            vec![1, 2, 4, 7, 8],
            vec![1, 2, 5, 6, 7],
        ];
        let mut results = recursive_set_covers(&universe, &subsets, limit);
        sort_results(&mut results);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);
    }
}
