use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    champions::Champion,
    traits::{ActiveTrait, Trait},
    solution::{Solution},
};

pub struct Team {
    pub champions: Vec<Champion>,
}

impl Team {
    // fn traits(&self) -> Vec<Trait> {
    //     self.champions
    //         .iter()
    //         .flat_map(|champ| champ.info().traits.iter().copied())
    //         .collect()
    // }
    pub fn from_names(names: Vec<&str>) -> Team {
        let champions = names
            .iter()
            .map(|name| Champion::from_str(name).expect("That wasn't an acceptable Champion name"))
            .collect();
        Team { champions }
    }
    fn trait_counts(&self) -> HashMap<Trait, u8> {
        let mut counts = HashMap::new();
        for champ in &self.champions {
            for &trait_ in champ.info().traits {
                *counts.entry(trait_).or_insert(0) += 1;
            }
        }
        counts
    }

    pub fn active_traits(&self) -> Vec<ActiveTrait> {
        self.trait_counts()
            .into_iter()
            .filter_map(|(trait_, count)| {
                ActiveTrait::new(trait_, count as usize, &trait_.levels())
            })
            .collect()
    }

    pub fn non_unique_traits(&self) -> Vec<ActiveTrait> {
        self.active_traits()
            .iter()
            .filter(|t| t.level > 1)
            .cloned()
            .collect()
    }

    fn generate_combination(&self, level: u8) -> Vec<Vec<Champion>> {
        let number_to_pick = level as usize - self.champions.len();
        let all_champs: HashSet<Champion> = Champion::iter().collect(); // Assuming this returns owned Champions
        let my_champs: HashSet<Champion> = self.champions.iter().cloned().collect(); // Clone to get owned values

        println!(
            "Looking for {} champs, passing in {:?}",
            number_to_pick, my_champs
        );

        // Collect remaining champions as owned values
        let remaining_champs: Vec<Champion> = all_champs.difference(&my_champs).cloned().collect();

        // println!("{}", fo)
        // Generate combinations from remaining champions
        let combos: Vec<Vec<Champion>> = remaining_champs
            .into_iter()
            .combinations(number_to_pick)
            .collect();

        combos
    }

    fn evaluate_combination(&self, combination: &Vec<Champion>) {
        let mut champs = self.champions.to_vec();
        champs.extend(combination);
        let new_team = Team { champions: champs };
        let non_unique_traits = new_team.non_unique_traits();
        if non_unique_traits.len() >= 7 {
            println!("{:?}", &combination)
            Solution {}
        }
    }

    pub fn find_solutions(&self) {
        self.generate_combination(7)
            .par_iter()
            .for_each(|combination| self.evaluate_combination(combination));
    }
}
