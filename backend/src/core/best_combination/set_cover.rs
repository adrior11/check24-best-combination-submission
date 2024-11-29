use crate::common::models::{StreamingOffer, StreamingPackage};
use std::collections::{BTreeMap, HashSet};

// TODO: Accumulate total costs and features
pub fn set_cover_best_combination(
    game_ids: &[u32],
    packages: &[StreamingPackage],
    offers: &[StreamingOffer],
) -> anyhow::Result<HashSet<StreamingPackage>> {
    let mut uncovered_games: HashSet<u32> = game_ids.iter().cloned().collect(); // Universe
    let mut bundle: HashSet<StreamingPackage> = HashSet::new(); // Output
    let mut package_to_offers: BTreeMap<u32, Vec<StreamingOffer>> = BTreeMap::new(); // Sets

    for offer in offers {
        package_to_offers
            .entry(offer.streaming_package_id)
            .or_default()
            .push(offer.to_owned());
    }

    while !uncovered_games.is_empty() {
        let mut best_package_id: Option<u32> = None;
        let mut best_cost_per_game: Option<f32> = None;

        for (&package_id, offers) in &package_to_offers {
            let package = packages
                .iter()
                .find(|p| p.streaming_package_id == package_id)
                .unwrap();

            let price = package
                .monthly_price_cents
                .unwrap_or(package.monthly_price_yearly_subscription_in_cents)
                as f32; // TODO: allow filter later on

            let newly_covered_games: HashSet<u32> = offers
                .iter()
                .map(|o| o.game_id)
                .collect::<HashSet<_>>()
                .intersection(&uncovered_games)
                .cloned()
                .collect();

            let new_covers = newly_covered_games.len();

            if new_covers == 0 {
                continue;
            }

            let cost_per_game = price / new_covers as f32;

            if best_cost_per_game.is_none() || cost_per_game < best_cost_per_game.unwrap() {
                best_cost_per_game = Some(cost_per_game);
                best_package_id = Some(package_id);
            }
        }

        if let Some(package_id) = best_package_id {
            let package = packages
                .iter()
                .find(|p| p.streaming_package_id == package_id)
                .unwrap();

            let offers = package_to_offers.remove(&package_id).unwrap();

            let newly_covered_games: HashSet<u32> = offers
                .iter()
                .map(|o| o.game_id)
                .collect::<HashSet<_>>()
                .intersection(&uncovered_games)
                .cloned()
                .collect();

            uncovered_games = &uncovered_games - &newly_covered_games;

            bundle.insert(package.to_owned());
        } else {
            break; // No package covers remaining games
        }
    }

    Ok(bundle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mongodb::bson::oid::ObjectId;

    fn create_streaming_package(
        streaming_package_id: u32,
        name: &str,
        monthly_price_cents: Option<u16>,
        monthly_price_yearly_subscription_in_cents: u16,
    ) -> StreamingPackage {
        let name = name.to_string();
        StreamingPackage {
            id: ObjectId::new(),
            name,
            streaming_package_id,
            monthly_price_cents,
            monthly_price_yearly_subscription_in_cents,
        }
    }

    fn create_streaming_offer(streaming_package_id: u32, game_id: u32) -> StreamingOffer {
        StreamingOffer {
            id: ObjectId::new(),
            game_id,
            streaming_package_id,
            live: 0,
            highlights: 0,
        }
    }

    fn create_streaming_offer_for_games(
        streaming_package_id: u32,
        game_ids: &[u32],
    ) -> Vec<StreamingOffer> {
        game_ids
            .iter()
            .map(|&game_id| create_streaming_offer(streaming_package_id, game_id))
            .collect()
    }

    fn filter_package_ids(packages: &HashSet<StreamingPackage>) -> HashSet<u32> {
        packages.iter().map(|p| p.streaming_package_id).collect()
    }

    #[test]
    fn test_simple_dataset() {
        let game_ids: Vec<u32> = (1..=10).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(50), 600),
            create_streaming_package(2, "P2", Some(30), 600),
            create_streaming_package(3, "P3", Some(20), 600),
            create_streaming_package(4, "P4", Some(15), 600),
            create_streaming_package(5, "P5", Some(10), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            create_streaming_offer_for_games(2, &[1, 2, 3, 4, 5]),
            create_streaming_offer_for_games(3, &[6, 7, 8, 9, 10]),
            create_streaming_offer_for_games(4, &[2, 4, 6, 8, 10]),
            create_streaming_offer_for_games(5, &[1, 3, 5, 7, 9]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [5, 4].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_overlapping_packages() {
        let game_ids: Vec<u32> = (1..=4).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(40), 600),
            create_streaming_package(2, "P2", Some(25), 600),
            create_streaming_package(3, "P3", Some(20), 600),
            create_streaming_package(4, "P4", Some(10), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2, 3]),
            create_streaming_offer_for_games(2, &[2, 3, 4]),
            create_streaming_offer_for_games(3, &[1]),
            create_streaming_offer_for_games(4, &[4]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [2, 3].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_no_overlap_packages() {
        let game_ids: Vec<u32> = (1..=3).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(15), 600),
            create_streaming_package(2, "P2", Some(10), 600),
            create_streaming_package(3, "P3", Some(5), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1]),
            create_streaming_offer_for_games(2, &[2]),
            create_streaming_offer_for_games(3, &[3]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1, 2, 3].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_avoid_redundant_packages() {
        // NOTE: This should lead two equal bundles as the output later on
        let game_ids: Vec<u32> = (1..=3).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(20), 600),
            create_streaming_package(2, "P2", Some(20), 600),
            create_streaming_package(3, "P3", Some(10), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2]),
            create_streaming_offer_for_games(2, &[1, 2]),
            create_streaming_offer_for_games(3, &[3]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1, 3].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_no_possible_coverage() {
        let game_ids = vec![1, 2];

        let packages = vec![create_streaming_package(1, "P1", Some(20), 600)];

        let offers = [create_streaming_offer_for_games(1, &[1])].concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids, "It's impossible to cover game 2");
    }

    #[test]
    fn test_varying_prices_and_coverage() {
        let game_ids: Vec<u32> = (1..=5).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(25), 600),
            create_streaming_package(2, "P2", Some(20), 600),
            create_streaming_package(3, "P3", Some(15), 600),
            create_streaming_package(4, "P2", Some(40), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2, 3]),
            create_streaming_offer_for_games(2, &[3, 4]),
            create_streaming_offer_for_games(3, &[4, 5]),
            create_streaming_offer_for_games(4, &[1, 2, 3, 4, 5]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1, 3].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_none_monthly_cost_packages() {
        let game_ids = vec![1];

        let packages = vec![
            create_streaming_package(1, "P1", None, 10),
            create_streaming_package(2, "P2", Some(15), 20),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1]),
            create_streaming_offer_for_games(2, &[1]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_greedy_algorithm_trade_off() {
        let game_ids: Vec<u32> = (1..=3).collect();

        let packages = vec![
            create_streaming_package(1, "P1", None, 10),
            create_streaming_package(2, "P2", Some(15), 20),
            create_streaming_package(3, "P3", None, 20),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1]),
            create_streaming_offer_for_games(2, &[1, 2]),
            create_streaming_offer_for_games(3, &[2, 3]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [2, 3].iter().cloned().collect();

        // NOTE: The greedy algorithm selects package 2 first due to its lower cost per game,
        // even though packages 1 and 3 would result in a lower total cost.
        assert_eq!(result_ids, expected_ids,);
    }

    #[test]
    fn test_all_packages_cover_all_games() {
        let game_ids: Vec<u32> = (1..=3).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(30), 360),
            create_streaming_package(2, "P2", Some(20), 240),
            create_streaming_package(3, "P3", Some(25), 300),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2, 3]),
            create_streaming_offer_for_games(2, &[1, 2, 3]),
            create_streaming_offer_for_games(3, &[1, 2, 3]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [2].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_complex_overlapping_subsets() {
        let game_ids: Vec<u32> = (1..=6).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(18), 600),
            create_streaming_package(2, "P2", Some(24), 600),
            create_streaming_package(3, "P3", Some(15), 600),
            create_streaming_package(4, "P4", Some(10), 600),
            create_streaming_package(5, "P5", Some(8), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1, 2, 3]),
            create_streaming_offer_for_games(2, &[3, 4, 5]),
            create_streaming_offer_for_games(3, &[5, 6]),
            create_streaming_offer_for_games(4, &[2, 4]),
            create_streaming_offer_for_games(5, &[6]),
        ]
        .concat();

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1, 3, 4].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);
    }

    #[test]
    #[should_panic]
    fn test_offers_with_invalid_package_ids() {
        let game_ids: Vec<u32> = vec![1, 2];

        let packages = vec![create_streaming_package(1, "P1", Some(10), 120)];

        // Offers refer to a non-existent package_id 2
        let offers = [create_streaming_offer_for_games(2, &[1])].concat();

        let _ = set_cover_best_combination(&game_ids, &packages, &offers);
    }

    #[test]
    fn test_stress_with_missing_prices_and_overlaps() {
        // Simulate 100 games
        let game_ids: Vec<u32> = (1..=100).collect();

        // Simulate 50 packages
        let mut packages = Vec::new();
        for i in 1..=50 {
            let monthly_price_cents = if i % 3 == 0 {
                None
            } else {
                Some((i * 2) as u16)
            };

            let yearly_price_cents = if i % 4 == 0 {
                (i * 30) as u16
            } else {
                (i * 20) as u16
            };

            packages.push(create_streaming_package(
                i,
                &format!("Package {}", i),
                monthly_price_cents,
                yearly_price_cents,
            ));
        }

        // Create offers with high overlaps, where only 75 games are covered
        let mut offers = Vec::new();
        for i in 1..=50 {
            let package_id = i;
            let start_game = (i % 25) + 1;
            let end_game = ((start_game + 50) % 100) + 1;
            let game_range: Vec<u32> = if start_game < end_game {
                (start_game..end_game).collect()
            } else {
                (start_game..=100).chain(1..end_game).collect()
            };
            offers.extend(create_streaming_offer_for_games(package_id, &game_range));
        }

        let result = set_cover_best_combination(&game_ids, &packages, &offers).unwrap();
        let result_ids: HashSet<u32> = filter_package_ids(&result);
        let expected_ids: HashSet<u32> = [1, 23, 25, 49].iter().cloned().collect();

        assert_eq!(result_ids, expected_ids);

        let total_cost: u32 = result
            .iter()
            .map(|p| {
                p.monthly_price_cents
                    .unwrap_or(p.monthly_price_yearly_subscription_in_cents) as u32
            })
            .sum();
        let expected_cost = 196;

        assert_eq!(total_cost, expected_cost);

        let mut covered_games = HashSet::new();
        for package_id in &result_ids {
            let package_offers: Vec<_> = offers
                .iter()
                .filter(|o| o.streaming_package_id == *package_id)
                .collect();
            for offer in package_offers {
                covered_games.insert(offer.game_id);
            }
        }

        assert_eq!(
            covered_games.len(),
            game_ids.len() - 25,
            "Only 75 out of the 100 games should be covered"
        );
    }
}
