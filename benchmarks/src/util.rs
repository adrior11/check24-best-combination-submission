use std::{collections::BTreeSet, fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RawSubset {
    pub id: usize,
    pub elements: Vec<usize>,
    pub cost: usize,
}

impl From<RawSubset> for Subset {
    fn from(raw: RawSubset) -> Self {
        Self {
            id: raw.id,
            elements: raw.elements.into_iter().collect(),
            cost: raw.cost,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subset {
    pub id: usize,
    pub elements: BTreeSet<usize>,
    pub cost: usize,
}

#[derive(Deserialize)]
pub struct UniverseWrapper {
    pub universe: BTreeSet<usize>,
}

pub fn build_test_data() -> (BTreeSet<usize>, Vec<Subset>) {
    let subset_file = File::open("data/subset_benchmark_data.json")
        .expect("Failed to open subset_benchmark_data.json");
    let subsets_raw: Vec<RawSubset> = serde_json::from_reader(BufReader::new(subset_file))
        .expect("Failed to parse subset_benchmark_data.json");
    let subsets: Vec<Subset> = subsets_raw.into_iter().map(Subset::from).collect();

    let universe_file = File::open("data/universe_benchmark_data.json")
        .expect("Failed to open universe_benchmark_data.json");
    let universe_wrappers: Vec<UniverseWrapper> =
        serde_json::from_reader(BufReader::new(universe_file))
            .expect("Failed to parse universe_benchmark_data.json");

    let universe = universe_wrappers
        .first()
        .map(|wrapper| wrapper.universe.clone())
        .expect("Universe file should contain at least one object with 'universe' field");

    (universe, subsets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_test_data() {
        let (universe, subsets) = build_test_data();
        assert!(!universe.is_empty());
        assert!(!subsets.is_empty());
    }
}
