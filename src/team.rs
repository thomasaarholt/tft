use pyo3::prelude::*;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    champions::Champion,
    solution::Solution,
    solution_iterator::SolutionIterator,
    traits::{ActiveTrait, Trait},
};

#[pyclass]
#[derive(Clone)]
pub struct Team {
    pub champions: Vec<Champion>,
}

#[pymethods]
impl Team {
    #[new]
    pub fn new(names: Vec<String>) -> Team {
        let champions = names
            .iter()
            .map(|name| Champion::from_str(name).expect("Invalid Champion name"))
            .collect();
        Team { champions }
    }
    pub fn find_solutions_py(&self, level: u8) -> SolutionIterator {
        SolutionIterator::new(self.clone(), level)
    }

    fn traits(&self) -> Vec<Trait> {
        self.champions
            .iter()
            .flat_map(|champ| champ.info().traits.iter().copied())
            .collect()
    }
}

impl Team {
    pub fn from_names(names: Vec<String>) -> Team {
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

    pub fn generate_combinations(&self, level: u8) -> Vec<HashSet<Champion>> {
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
            .into_iter()
            .map(|vec_champ| vec_champ.into_iter().collect::<HashSet<Champion>>())
            .collect()
    }

    pub fn evaluate_combination(&self, combination: &HashSet<Champion>) -> Option<Solution> {
        let mut champs = self.champions.clone();
        champs.extend(combination);
        let new_team = Team { champions: champs };
        let non_unique_traits = new_team.non_unique_traits();
        if non_unique_traits.len() >= 7 {
            let champions: HashSet<Champion> = new_team.champions.iter().copied().collect();
            let my_champions = HashSet::from_iter(self.champions.clone());
            let missing_champions: HashSet<Champion> =
                champions.difference(&my_champions).cloned().collect();

            Some(Solution {
                champions,
                missing_champions,
                traits: new_team.traits(),
            })
        } else {
            None
        }
    }

    pub fn find_solutions(&self, level: u8) -> Vec<Solution> {
        self.generate_combinations(level)
            .par_iter()
            .filter_map(|combination| self.evaluate_combination(combination))
            .collect()
    }
}
