use std::collections::BTreeSet;

use libs::models::dtos::{BestCombinationDto, BestCombinationSubsetDto};

pub fn map_to_best_combination_dto(
    current_cover: &[usize],
    subsets: &[BestCombinationSubsetDto],
    universe: &BTreeSet<usize>,
) -> BestCombinationDto {
    let mut packages = Vec::new();
    let mut combined_monthly_price_cents = 0;
    let mut combined_monthly_price_yearly_subscription_in_cents = 0;
    let mut covered: BTreeSet<usize> = BTreeSet::new();

    for subset in subsets {
        if current_cover.contains(&subset.streaming_package_id) {
            packages.push(subset.streaming_package_id);
            combined_monthly_price_cents += subset.monthly_price_cents.unwrap_or(0);
            combined_monthly_price_yearly_subscription_in_cents +=
                subset.monthly_price_yearly_subscription_in_cents;

            for e in &subset.elements {
                if universe.contains(e) {
                    covered.insert(*e);
                }
            }
        }
    }

    let coverage = (covered.len() as f64 / universe.len() as f64 * 100.0).round() as u8;

    packages.sort();

    BestCombinationDto {
        packages,
        combined_monthly_price_cents,
        combined_monthly_price_yearly_subscription_in_cents,
        coverage,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapper() {
        let current_cover = [1, 2, 4];
        let subsets = vec![
            BestCombinationSubsetDto {
                streaming_package_id: 1,
                elements: BTreeSet::from([1]),
                monthly_price_cents: Some(10),
                monthly_price_yearly_subscription_in_cents: 10,
            },
            BestCombinationSubsetDto {
                streaming_package_id: 2,
                elements: BTreeSet::from([1, 3]),
                monthly_price_cents: None,
                monthly_price_yearly_subscription_in_cents: 10,
            },
        ];
        let universe = BTreeSet::from([1, 2, 3]);

        let result = map_to_best_combination_dto(&current_cover, &subsets, &universe);
        let expected = BestCombinationDto {
            packages: vec![1, 2],
            combined_monthly_price_cents: 10,
            combined_monthly_price_yearly_subscription_in_cents: 20,
            coverage: 67,
        };

        assert_eq!(result, expected);
    }
}
