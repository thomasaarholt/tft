use pyo3::prelude::*;
use std::{collections::HashSet, fmt};

use crate::{champions::Champion, traits::ActiveTrait};

#[pyclass(get_all, name = "SolutionRust")]
#[derive(Clone)]
pub struct Solution {
    pub champions: HashSet<Champion>,
    pub missing_champions: HashSet<Champion>,
    pub traits: Vec<ActiveTrait>,
}

#[pymethods]
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

    // Add the __str__ and __repr__ methods
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Solution({})", self))
    }
}
impl fmt::Display for Solution {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut sorted_champs: Vec<Champion> = self.missing_champions.iter().copied().collect();
        sorted_champs.sort();
        for champ in sorted_champs {
            write!(f, "{}, ", champ.to_owned())?
        }
        Ok(())
    }
}
