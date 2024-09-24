struct Solution {
    champions: Vec<Champion>,
    missing_champions: Vec<Champion>,
    traits: Vec<Trait>,
}

impl Solution {
    pub fn cost(&self) -> u8 {
        self.champions.iter().map(|champ| champ.info().cost).sum()
    }
    pub fn missing_cost(&self) -> u8 {
        self.missing_champions
            .iter()
            .map(|champ| champ.info().cost)
            .sum()
    }
}
