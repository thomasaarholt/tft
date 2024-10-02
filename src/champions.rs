use itertools::Format;
use pyo3::prelude::*;
use std::{
    cmp::Ordering,
    fmt::{self, format},
};

use strum::{EnumIter, EnumString};

use crate::traits::Trait;

#[derive(Debug, Clone, Copy)]
pub struct Info {
    pub traits: &'static [Trait],
    pub cost: u8,
}

#[pyclass(name = "ChampionRust", eq, eq_int, frozen, hash)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumString)]
pub enum Champion {
    Ahri,
    Akali,
    Ashe,
    Bard,
    Blitzcrank,
    Briar,
    Camille,
    Cassiopeia,
    Diana,
    Elise,
    Ezreal,
    Fiora,
    Galio,
    Gwen,
    Hecarim,
    Hwei,
    Jax,
    Jayce,
    Jinx,
    Kalista,
    Karma,
    Kassadin,
    Katarina,
    KogMaw,
    Lillia,
    Milio,
    Mordekaiser,
    Morgana,
    Nami,
    Nasus,
    Neeko,
    Nilah,
    Nomsy,
    Norra,
    Nunu,
    Olaf,
    Poppy,
    Rakan,
    Rumble,
    Ryze,
    Seraphine,
    Shen,
    Shyvana,
    Smolder,
    Soraka,
    Swain,
    Syndra,
    TahmKench,
    Taric,
    Tristana,
    Twitch,
    Varus,
    Veigar,
    Vex,
    Warwick,
    Wukong,
    Xerath,
    Ziggs,
    Zilean,
    Zoe,
}

impl Champion {
    pub fn info(self) -> &'static Info {
        match self {
            Champion::Ahri => &Info {
                traits: &[Trait::Arcana, Trait::Scholar],
                cost: 2,
            },
            Champion::Akali => &Info {
                traits: &[Trait::Pyro, Trait::Multistriker, Trait::Warrior],
                cost: 2,
            },
            Champion::Ashe => &Info {
                traits: &[Trait::Eldritch, Trait::Multistriker],
                cost: 1,
            },
            Champion::Bard => &Info {
                traits: &[Trait::Sugarcraft, Trait::Preserver, Trait::Scholar],
                cost: 3,
            },
            Champion::Blitzcrank => &Info {
                traits: &[Trait::Honeymancy, Trait::Vanguard],
                cost: 1,
            },
            Champion::Briar => &Info {
                traits: &[Trait::Eldritch, Trait::Ravenous, Trait::Shapeshifter],
                cost: 5,
            },
            Champion::Camille => &Info {
                traits: &[Trait::Chrono, Trait::Multistriker],
                cost: 5,
            },
            Champion::Cassiopeia => &Info {
                traits: &[Trait::Witchcraft, Trait::Incantor],
                cost: 2,
            },
            Champion::Diana => &Info {
                traits: &[Trait::Frost, Trait::Bastion],
                cost: 5,
            },
            Champion::Elise => &Info {
                traits: &[Trait::Eldritch, Trait::Shapeshifter],
                cost: 1,
            },
            Champion::Ezreal => &Info {
                traits: &[Trait::Portal, Trait::Blaster],
                cost: 3,
            },
            Champion::Fiora => &Info {
                traits: &[Trait::Witchcraft, Trait::Warrior],
                cost: 4,
            },
            Champion::Galio => &Info {
                traits: &[Trait::Portal, Trait::Mage, Trait::Vanguard],
                cost: 2,
            },
            Champion::Gwen => &Info {
                traits: &[Trait::Sugarcraft, Trait::Warrior],
                cost: 4,
            },
            Champion::Hecarim => &Info {
                traits: &[Trait::Arcana, Trait::Bastion, Trait::Multistriker],
                cost: 3,
            },
            Champion::Hwei => &Info {
                traits: &[Trait::Frost, Trait::Blaster],
                cost: 3,
            },
            Champion::Jax => &Info {
                traits: &[Trait::Chrono, Trait::Multistriker],
                cost: 1,
            },
            Champion::Jayce => &Info {
                traits: &[Trait::Portal, Trait::Shapeshifter],
                cost: 1,
            },
            Champion::Jinx => &Info {
                traits: &[Trait::Sugarcraft, Trait::Hunter],
                cost: 3,
            },
            Champion::Kalista => &Info {
                traits: &[Trait::Faerie, Trait::Multistriker],
                cost: 4,
            },
            Champion::Karma => &Info {
                traits: &[Trait::Chrono, Trait::Incantor],
                cost: 4,
            },
            Champion::Kassadin => &Info {
                traits: &[Trait::Portal, Trait::Multistriker],
                cost: 2,
            },
            Champion::Katarina => &Info {
                traits: &[Trait::Faerie, Trait::Warrior],
                cost: 3,
            },
            Champion::KogMaw => &Info {
                traits: &[Trait::Honeymancy, Trait::Hunter],
                cost: 2,
            },
            Champion::Lillia => &Info {
                traits: &[Trait::Faerie, Trait::Bastion],
                cost: 1,
            },
            Champion::Milio => &Info {
                traits: &[Trait::Faerie, Trait::Scholar],
                cost: 5,
            },
            Champion::Mordekaiser => &Info {
                traits: &[Trait::Eldritch, Trait::Vanguard],
                cost: 3,
            },
            Champion::Morgana => &Info {
                traits: &[Trait::Witchcraft, Trait::BatQueen, Trait::Preserver],
                cost: 5,
            },
            Champion::Nami => &Info {
                traits: &[Trait::Eldritch, Trait::Mage],
                cost: 4,
            },
            Champion::Nasus => &Info {
                traits: &[Trait::Pyro, Trait::Shapeshifter],
                cost: 4,
            },
            Champion::Neeko => &Info {
                traits: &[Trait::Witchcraft, Trait::Shapeshifter],
                cost: 3,
            },
            Champion::Nilah => &Info {
                traits: &[Trait::Eldritch, Trait::Warrior],
                cost: 2,
            },
            Champion::Nomsy => &Info {
                traits: &[Trait::Dragon, Trait::Hunter],
                cost: 1,
            },
            Champion::Norra => &Info {
                traits: &[Trait::Portal, Trait::BestFriends, Trait::Mage],
                cost: 5,
            },
            Champion::Nunu => &Info {
                traits: &[Trait::Honeymancy, Trait::Bastion],
                cost: 2,
            },
            Champion::Olaf => &Info {
                traits: &[Trait::Frost, Trait::Hunter],
                cost: 4,
            },
            Champion::Poppy => &Info {
                traits: &[Trait::Witchcraft, Trait::Bastion],
                cost: 1,
            },
            Champion::Rakan => &Info {
                traits: &[Trait::Faerie, Trait::Preserver],
                cost: 4,
            },
            Champion::Rumble => &Info {
                traits: &[Trait::Sugarcraft, Trait::Blaster, Trait::Vanguard],
                cost: 2,
            },
            Champion::Ryze => &Info {
                traits: &[Trait::Portal, Trait::Scholar],
                cost: 4,
            },
            Champion::Seraphine => &Info {
                traits: &[Trait::Faerie, Trait::Mage],
                cost: 1,
            },
            Champion::Shen => &Info {
                traits: &[Trait::Pyro, Trait::Bastion],
                cost: 3,
            },
            Champion::Shyvana => &Info {
                traits: &[Trait::Dragon, Trait::Shapeshifter],
                cost: 2,
            },
            Champion::Smolder => &Info {
                traits: &[Trait::Dragon, Trait::Blaster],
                cost: 5,
            },
            Champion::Soraka => &Info {
                traits: &[Trait::Sugarcraft, Trait::Mage],
                cost: 1,
            },
            Champion::Swain => &Info {
                traits: &[Trait::Frost, Trait::Shapeshifter],
                cost: 3,
            },
            Champion::Syndra => &Info {
                traits: &[Trait::Eldritch, Trait::Incantor],
                cost: 2,
            },
            Champion::TahmKench => &Info {
                traits: &[Trait::Arcana, Trait::Vanguard],
                cost: 4,
            },
            Champion::Taric => &Info {
                traits: &[Trait::Portal, Trait::Bastion],
                cost: 4,
            },
            Champion::Tristana => &Info {
                traits: &[Trait::Faerie, Trait::Blaster],
                cost: 2,
            },
            Champion::Twitch => &Info {
                traits: &[Trait::Frost, Trait::Hunter],
                cost: 1,
            },
            Champion::Varus => &Info {
                traits: &[Trait::Pyro, Trait::Blaster],
                cost: 4,
            },
            Champion::Veigar => &Info {
                traits: &[Trait::Honeymancy, Trait::Mage],
                cost: 3,
            },
            Champion::Vex => &Info {
                traits: &[Trait::Chrono, Trait::Mage],
                cost: 3,
            },
            Champion::Warwick => &Info {
                traits: &[Trait::Frost, Trait::Vanguard],
                cost: 1,
            },
            Champion::Wukong => &Info {
                traits: &[Trait::Druid],
                cost: 3,
            },
            Champion::Xerath => &Info {
                traits: &[Trait::Arcana, Trait::Ascendant],
                cost: 5,
            },
            Champion::Ziggs => &Info {
                traits: &[Trait::Honeymancy, Trait::Incantor],
                cost: 1,
            },
            Champion::Zilean => &Info {
                traits: &[Trait::Chrono, Trait::Frost, Trait::Preserver],
                cost: 2,
            },
            Champion::Zoe => &Info {
                traits: &[Trait::Portal, Trait::Witchcraft, Trait::Scholar],
                cost: 1,
            },
        }
    }
}

impl fmt::Display for Champion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:?}", self.info().cost, self)
    }
}

impl Ord for Champion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.info().cost.cmp(&other.info().cost) {
            Ordering::Equal => format!("{:?}", self).cmp(&format!("{:?}", other)),
            other => other,
        }
    }
}

impl PartialOrd for Champion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[pymethods]
impl Champion {
    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}
