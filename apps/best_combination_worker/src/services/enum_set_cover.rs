use anyhow::Context;
use libs::models::schemas::{StreamingOfferSchema, StreamingPackageSchema};
use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, BinaryHeap, HashSet};

pub type BestCombinationResult = HashSet<StreamingPackageSchema>;

pub fn get_enumerated_best_combinations(
    game_ids: &[u32],
    offers: &[StreamingOfferSchema],
    packages: &[StreamingPackageSchema],
    limit: usize,
) -> anyhow::Result<Vec<BestCombinationResult>> {
    let uncovered_games: HashSet<u32> = game_ids.iter().cloned().collect();
    let mut package_to_offers: BTreeMap<u32, Vec<StreamingOfferSchema>> = BTreeMap::new();

    // Map offers to their respective packages
    for offer in offers {
        package_to_offers
            .entry(offer.streaming_package_id)
            .or_default()
            .push(offer.to_owned());
    }

    // Priority queue to explore combinations by cost
    let mut heap: BinaryHeap<(
        OrderedFloat<f32>,
        Vec<u32>,                    // Vec for covered games
        Vec<StreamingPackageSchema>, // Vec for combination of packages
    )> = BinaryHeap::new();

    // Add each package as a starting point
    for (&package_id, offers) in &package_to_offers {
        let package = packages
            .iter()
            .find(|p| p.streaming_package_id == package_id)
            .context("Offers streaming_package_id is invalid")?;

        let price = package
            .monthly_price_cents
            .unwrap_or(package.monthly_price_yearly_subscription_in_cents)
            as f32;

        let covered_games: Vec<u32> = offers.iter().map(|o| o.game_id).collect();

        heap.push((
            OrderedFloat(price),
            covered_games.clone(),
            vec![package.to_owned()],
        ));
    }

    log::debug!("Starting heap: {:?}", heap);
    dbg!(&heap);

    let mut results = Vec::new();

    // Explore combinations
    while let Some((total_cost, covered_games, current_combination)) = heap.pop() {
        log::debug!("total_cost: {:?}", &total_cost);
        log::debug!("covered_games: {:?}", &covered_games);
        log::debug!("current_combination: {:?}", &current_combination);

        if results.len() >= limit {
            break; // Stop when the limit is reached
        }

        // Convert Vec<u32> back to HashSet for subset check
        let covered_set: HashSet<_> = covered_games.iter().cloned().collect();

        // If this combination covers all games, add it to the results
        if uncovered_games.is_subset(&covered_set) {
            let result_set: HashSet<_> = current_combination.clone().into_iter().collect();
            results.push(result_set);
        }

        // Expand current combination with other packages
        for (&package_id, offers) in &package_to_offers {
            if current_combination
                .iter()
                .any(|p| p.streaming_package_id == package_id)
            {
                continue; // Skip already included packages
            }

            let package = packages
                .iter()
                .find(|p| p.streaming_package_id == package_id)
                .context("Offers streaming_package_id is invalid")?;

            let price = package
                .monthly_price_cents
                .unwrap_or(package.monthly_price_yearly_subscription_in_cents)
                as f32;

            let new_covered_games: Vec<u32> = offers.iter().map(|o| o.game_id).collect();

            let mut new_combination = current_combination.clone();
            new_combination.push(package.clone());

            let new_covered_union: Vec<u32> = covered_games
                .iter()
                .chain(new_covered_games.iter())
                .cloned()
                .collect();

            heap.push((
                OrderedFloat(total_cost.into_inner() + price),
                new_covered_union,
                new_combination,
            ));
        }
    }

    Ok(results)
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
    ) -> StreamingPackageSchema {
        let name = name.to_string();
        StreamingPackageSchema {
            id: ObjectId::new(),
            name,
            streaming_package_id,
            monthly_price_cents,
            monthly_price_yearly_subscription_in_cents,
        }
    }

    fn create_streaming_offer(streaming_package_id: u32, game_id: u32) -> StreamingOfferSchema {
        StreamingOfferSchema {
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
    ) -> Vec<StreamingOfferSchema> {
        game_ids
            .iter()
            .map(|&game_id| create_streaming_offer(streaming_package_id, game_id))
            .collect()
    }

    fn filter_result_ids(results: &[BestCombinationResult]) -> Vec<HashSet<u32>> {
        results
            .iter()
            .map(|set| {
                set.iter()
                    .map(|p| p.streaming_package_id)
                    .collect::<HashSet<u32>>()
            })
            .collect()
    }

    fn vec_of_hashsets(nested: &[&[u32]]) -> Vec<HashSet<u32>> {
        nested
            .iter()
            .map(|inner| inner.iter().cloned().collect::<HashSet<u32>>())
            .collect()
    }

    #[test]
    fn test_trivial_behaviour() {
        libs::logging::init_logging();

        let game_ids: Vec<u32> = (1..=2).collect();

        let packages = vec![
            create_streaming_package(1, "P1", Some(15), 600),
            create_streaming_package(2, "P2", Some(10), 600),
            create_streaming_package(3, "P3", Some(5), 600),
        ];

        let offers = [
            create_streaming_offer_for_games(1, &[1]),
            create_streaming_offer_for_games(2, &[1]),
            create_streaming_offer_for_games(3, &[2]),
        ]
        .concat();

        {
            let limit = 1;

            let result =
                get_enumerated_best_combinations(&game_ids, &offers, &packages, limit).unwrap();
            let result_ids = filter_result_ids(&result);
            let expected_ids = vec_of_hashsets(&[[2, 3].as_ref()]);

            assert_eq!(result_ids, expected_ids);
        }
    }
}

// pub fn get_enumerated_combinations_2(
//     game_ids: &[u32],
//     offers: &[StreamingOfferSchema],
//     packages: &[StreamingPackageSchema],
//     limit: usize,
// ) -> anyhow::Result<Vec<BestCombinationResult>> {
//     let mut uncovered_games: HashSet<u32> = game_ids.iter().cloned().collect();
//     let mut package_to_offers: BTreeMap<u32, Vec<StreamingOfferSchema>> = BTreeMap::new();
//
//     // Map offers to their respective packages
//     for offer in offers {
//         package_to_offers
//             .entry(offer.streaming_package_id)
//             .or_default()
//             .push(offer.to_owned());
//     }
//
//     // Priority queue to explore combinations by cost
//     let mut heap: BinaryHeap<Reverse<(OrderedFloat<f32>, HashSet<u32>, BestCombinationResult)>> =
//         BinaryHeap::new();
//
//     // Add each package as a starting point
//     for (&package_id, offers) in &package_to_offers {
//         let package = packages
//             .iter()
//             .find(|p| p.streaming_package_id == package_id)
//             .context("Offers streaming_package_id is invalid")?;
//
//         let price = package
//             .monthly_price_cents
//             .unwrap_or(package.monthly_price_yearly_subscription_in_cents)
//             as f32;
//
//         let covered_games: HashSet<u32> = offers.iter().map(|o| o.game_id).collect();
//
//         heap.push(Reverse((OrderedFloat(price), covered_games.clone(), {
//             let mut result = HashSet::new();
//             result.insert(package.to_owned());
//             result
//         })));
//     }
//
//     let mut results = Vec::new();
//
//     // Explore combinations
//     while let Some(Reverse((total_cost, covered_games, current_combination))) = heap.pop() {
//         if results.len() >= limit {
//             break; // Stop when the limit is reached
//         }
//
//         // If this combination covers all games, add it to the results
//         if uncovered_games.is_subset(&covered_games) {
//             results.push(current_combination.clone());
//         }
//
//         // Expand current combination with other packages
//         for (&package_id, offers) in &package_to_offers {
//             if current_combination
//                 .iter()
//                 .any(|p| p.streaming_package_id == package_id)
//             {
//                 continue; // Skip already included packages
//             }
//
//             let package = packages
//                 .iter()
//                 .find(|p| p.streaming_package_id == package_id)
//                 .context("Offers streaming_package_id is invalid")?;
//
//             let price = package
//                 .monthly_price_cents
//                 .unwrap_or(package.monthly_price_yearly_subscription_in_cents)
//                 as f32;
//
//             let covered_games_new: HashSet<u32> = offers.iter().map(|o| o.game_id).collect();
//
//             let mut new_combination = current_combination.clone();
//             new_combination.insert(package.clone());
//
//             heap.push(Reverse((
//                 OrderedFloat(total_cost.into_inner() + price),
//                 covered_games.union(&covered_games_new).cloned().collect(),
//                 new_combination,
//             )));
//         }
//     }
//
//     Ok(results)
// }
