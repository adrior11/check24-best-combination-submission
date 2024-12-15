use crate::util::Subset;
use core::f64;
use std::collections::{BTreeSet, HashSet};

pub fn iterative_set_cover(universe: &BTreeSet<usize>, subsets: &[Subset]) -> HashSet<usize> {
    let mut uncovered: BTreeSet<usize> = universe.iter().cloned().collect();
    let mut chosen_subsets = HashSet::new();

    while !uncovered.is_empty() {
        let mut best_subset_id: Option<usize> = None;
        let mut best_ratio: f64 = f64::INFINITY;

        for subset in subsets {
            let newly_covered: HashSet<usize> =
                subset.elements.intersection(&uncovered).cloned().collect();

            if newly_covered.is_empty() {
                continue;
            }

            let ratio = subset.cost as f64 / newly_covered.len() as f64;

            if ratio < best_ratio {
                best_ratio = ratio;
                best_subset_id = Some(subset.id);
            }
        }

        if let Some(id) = best_subset_id {
            let chosen_subset = subsets.iter().find(|s| s.id == id).unwrap();

            for element in &chosen_subset.elements {
                uncovered.remove(element);
            }

            chosen_subsets.insert(id);
        } else {
            break; // No package covers remaining games
        }
    }

    chosen_subsets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_universe() {
        let universe = BTreeSet::new();
        let subsets = vec![];

        let expected_cover = HashSet::new();
        let results = iterative_set_cover(&universe, &subsets);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_no_subsets() {
        let universe = BTreeSet::from([1, 2]);
        let subsets = vec![];

        let expected_cover = HashSet::new();
        let results = iterative_set_cover(&universe, &subsets);
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

        let expected_cover = HashSet::from([1]);
        let results = iterative_set_cover(&universe, &subsets);
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

        // This variant of the set cover, returns the next best possible
        let expected_cover = HashSet::from([1, 2]);
        let results = iterative_set_cover(&universe, &subsets);
        assert_eq!(results, expected_cover);
    }

    #[test]
    fn test_single_element_universe() {
        let universe = BTreeSet::from([1]);
        let subsets = vec![
            Subset {
                id: 1,
                elements: BTreeSet::from([1]),
                cost: 10,
            },
            Subset {
                id: 2,
                elements: BTreeSet::from([2]),
                cost: 5,
            },
        ];

        let expected_cover = HashSet::from([1]);
        let results = iterative_set_cover(&universe, &subsets);
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

        let expected_cover = HashSet::from([1]);
        let results = iterative_set_cover(&universe, &subsets);
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
            }, // Same as above
            Subset {
                id: 3,
                elements: BTreeSet::from([2]),
                cost: 5,
            },
        ];

        // Covers {1,3} and {2,3} as subsets 1 and 2 are identical in coverage and cost,
        // but the algorithm will nonetheless return only the first valid set cover.
        let expected_cover = HashSet::from([1, 3]);
        let results = iterative_set_cover(&universe, &subsets);
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

        let expected_cover = HashSet::from([1, 7, 6, 3, 5]);
        let results = iterative_set_cover(&universe, &subsets);
        assert!(!results.is_empty(),);
        assert_eq!(results, expected_cover);
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

        // Element 6 of the universe is never being covered, as S3 is missing.
        let expected_cover = HashSet::from([1, 5, 6, 7]);
        let results = iterative_set_cover(&universe, &subsets);

        assert!(
            !results.is_empty(),
            "Should find at least one cover for a large universe"
        );

        assert_eq!(results, expected_cover);
    }
}
