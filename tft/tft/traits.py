from dataclasses import dataclass
from enum import StrEnum, auto
from typing import Literal, TypeAlias


class Trait(StrEnum):
    Arcana = auto()
    Ascendant = auto()
    Bastion = auto()
    BatQueen = "bat queen"
    BestFriends = "best friends"
    Blaster = auto()
    Chrono = auto()
    Dragon = auto()
    Druid = auto()
    Eldritch = auto()
    Faerie = auto()
    Frost = auto()
    Honeymancy = auto()
    Hunter = auto()
    Incantor = auto()
    Mage = auto()
    Multistriker = auto()
    Portal = auto()
    Preserver = auto()
    Pyro = auto()
    Ravenous = auto()
    Scholar = auto()
    Shapeshifter = auto()
    Sugarcraft = auto()
    Vanguard = auto()
    Warrior = auto()
    Witchcraft = auto()

    def __repr__(self) -> str:
        return self.name


classes_data: dict[Trait, list[int]] = {
    Trait.Ascendant: [1],
    Trait.Bastion: [2, 4, 6, 8],
    Trait.BatQueen: [1],
    Trait.BestFriends: [1],
    Trait.Blaster: [2, 4, 6],
    Trait.Hunter: [2, 4, 6],
    Trait.Incantor: [2, 4],
    Trait.Mage: [3, 5, 7, 10],
    Trait.Multistriker: [3, 5, 7, 9],
    Trait.Preserver: [2, 3, 4, 5],
    Trait.Scholar: [2, 4, 6],
    Trait.Shapeshifter: [2, 4, 6, 8],
    Trait.Vanguard: [2, 4, 6],
    Trait.Warrior: [2, 4, 6],
}

origins_data: dict[Trait, list[int]] = {
    Trait.Arcana: [2, 3, 4, 5],
    Trait.Chrono: [2, 4, 6],
    Trait.Dragon: [2, 3],
    Trait.Druid: [1],
    Trait.Eldritch: [3, 5, 7, 10],
    Trait.Faerie: [3, 5, 7, 9],
    Trait.Frost: [3, 5, 7, 9],
    Trait.Honeymancy: [3, 5, 7],
    Trait.Portal: [3, 6, 8, 10],
    Trait.Pyro: [2, 3, 4, 5],
    Trait.Ravenous: [1],
    Trait.Sugarcraft: [2, 4, 6],
    Trait.Witchcraft: [2, 4, 6, 8],
}

levels = classes_data | origins_data


@dataclass(frozen=True)
class ActiveTrait:
    trait: Trait
    level: int

    @classmethod
    def new(cls, trait: Trait, count: int):
        level = get_nearest_lower_level(levels[trait], count)
        if level is None:
            return None
        return cls(trait, level)

    def __repr__(self):
        return f"{self.trait.name.capitalize()}: {self.level}"

    # def __hash__(self):
    #     return


def get_nearest_lower_level(levels: list[int], count: int):
    nearest_level: int | None = None
    for level in levels:
        if level <= count:
            nearest_level = level
        else:
            break
    return nearest_level


TraitName: TypeAlias = Literal[
    "Arcana",
    "Ascendant",
    "Bastion",
    "BatQueen",
    "BestFriends",
    "Blaster",
    "Chrono",
    "Dragon",
    "Druid",
    "Eldritch",
    "Faerie",
    "Frost",
    "Honeymancy",
    "Hunter",
    "Incantor",
    "Mage",
    "Multistriker",
    "Portal",
    "Preserver",
    "Pyro",
    "Ravenous",
    "Scholar",
    "Shapeshifter",
    "Sugarcraft",
    "Vanguard",
    "Warrior",
    "Witchcraft",
]
