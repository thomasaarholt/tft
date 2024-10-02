use std::fmt;

use pyo3::prelude::*;
use strum::EnumString;

#[pyclass(name = "TraitRust", get_all, eq, eq_int)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString)]
pub enum Trait {
    Ascendant,
    Bastion,
    BatQueen,
    BestFriends,
    Blaster,
    Hunter,
    Incantor,
    Mage,
    Multistriker,
    Preserver,
    Scholar,
    Shapeshifter,
    Vanguard,
    Warrior,
    Arcana,
    Chrono,
    Dragon,
    Druid,
    Eldritch,
    Faerie,
    Frost,
    Honeymancy,
    Portal,
    Pyro,
    Ravenous,
    Sugarcraft,
    Witchcraft,
}
impl fmt::Display for Trait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Trait {
    pub fn levels(&self) -> Vec<u8> {
        match self {
            Trait::Ascendant => vec![1],
            Trait::Bastion => vec![2, 4, 6, 8],
            Trait::BatQueen => vec![1],
            Trait::BestFriends => vec![1],
            Trait::Blaster => vec![2, 4, 6],
            Trait::Hunter => vec![2, 4, 6],
            Trait::Incantor => vec![2, 4],
            Trait::Mage => vec![3, 5, 7, 10],
            Trait::Multistriker => vec![3, 5, 7, 9],
            Trait::Preserver => vec![2, 3, 4, 5],
            Trait::Scholar => vec![2, 4, 6],
            Trait::Shapeshifter => vec![2, 4, 6, 8],
            Trait::Vanguard => vec![2, 4, 6],
            Trait::Warrior => vec![2, 4, 6],
            Trait::Arcana => vec![2, 3, 4, 5],
            Trait::Chrono => vec![2, 4, 6],
            Trait::Dragon => vec![2, 3],
            Trait::Druid => vec![1],
            Trait::Eldritch => vec![3, 5, 7, 10],
            Trait::Faerie => vec![3, 5, 7, 9],
            Trait::Frost => vec![3, 5, 7, 9],
            Trait::Honeymancy => vec![3, 5, 7],
            Trait::Portal => vec![3, 6, 8, 10],
            Trait::Pyro => vec![2, 3, 4, 5],
            Trait::Ravenous => vec![1],
            Trait::Sugarcraft => vec![2, 4, 6],
            Trait::Witchcraft => vec![2, 4, 6, 8],
        }
    }
}
#[pymethods]
impl Trait {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(format!("{}", self))
    }
}

#[pyclass(name = "ActiveTraitRust", get_all)]
#[derive(Debug, Clone, Copy, Hash)]
pub struct ActiveTrait {
    pub trait_: Trait,
    pub level: u8,
}

fn get_nearest_lower_level(levels: &[u8], count: u8) -> Option<u8> {
    levels
        .iter()
        .cloned()
        .filter(|&level| level <= count)
        .last()
}

impl ActiveTrait {
    pub fn new(champtrait: Trait, count: usize, levels: &[u8]) -> Option<Self> {
        let level = get_nearest_lower_level(levels, count as u8)?;
        Some(ActiveTrait {
            trait_: champtrait,
            level,
        })
    }
}
