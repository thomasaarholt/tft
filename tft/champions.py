from dataclasses import dataclass
from enum import Enum
from typing import Literal, TypeAlias
import typing

from tft.traits import Trait


@dataclass
class ChampionTraits:
    traits: set[Trait]
    cost: int


class Champion(ChampionTraits, Enum):
    Ahri = {Trait.Arcana, Trait.Scholar}, 2
    Akali = {Trait.Pyro, Trait.Multistriker, Trait.Warrior}, 2
    Ashe = {Trait.Eldritch, Trait.Multistriker}, 1
    Bard = {Trait.Sugarcraft, Trait.Preserver, Trait.Scholar}, 3
    Blitzcrank = {Trait.Honeymancy, Trait.Vanguard}, 1
    Briar = {Trait.Eldritch, Trait.Ravenous, Trait.Shapeshifter}, 5
    Camille = {Trait.Chrono, Trait.Multistriker}, 5
    Cassiopeia = {Trait.Witchcraft, Trait.Incantor}, 2
    Diana = {Trait.Frost, Trait.Bastion}, 5
    Elise = {Trait.Eldritch, Trait.Shapeshifter}, 1
    Ezreal = {Trait.Portal, Trait.Blaster}, 3
    Fiora = {Trait.Witchcraft, Trait.Warrior}, 4
    Galio = {Trait.Portal, Trait.Mage, Trait.Vanguard}, 2
    Gwen = {Trait.Sugarcraft, Trait.Warrior}, 4
    Hecarim = {Trait.Arcana, Trait.Bastion, Trait.Multistriker}, 3
    Hwei = {Trait.Frost, Trait.Blaster}, 3
    Jax = {Trait.Chrono, Trait.Multistriker}, 1
    Jayce = {Trait.Portal, Trait.Shapeshifter}, 1
    Jinx = {Trait.Sugarcraft, Trait.Hunter}, 3
    Kalista = {Trait.Faerie, Trait.Multistriker}, 4
    Karma = {Trait.Chrono, Trait.Incantor}, 4
    Kassadin = {Trait.Portal, Trait.Multistriker}, 2
    Katarina = {Trait.Faerie, Trait.Warrior}, 3
    KogMaw = {Trait.Honeymancy, Trait.Hunter}, 2
    Lillia = {Trait.Faerie, Trait.Bastion}, 1
    Milio = {Trait.Faerie, Trait.Scholar}, 5
    Mordekaiser = {Trait.Eldritch, Trait.Vanguard}, 3
    Morgana = {Trait.Witchcraft, Trait.BatQueen, Trait.Preserver}, 5
    Nami = {Trait.Eldritch, Trait.Mage}, 4
    Nasus = {Trait.Pyro, Trait.Shapeshifter}, 4
    Neeko = {Trait.Witchcraft, Trait.Shapeshifter}, 3
    Nilah = {Trait.Eldritch, Trait.Warrior}, 2
    Nomsy = {Trait.Dragon, Trait.Hunter}, 1
    Norra = {Trait.Portal, Trait.BestFriends, Trait.Mage}, 5
    Nunu = {Trait.Honeymancy, Trait.Bastion}, 2
    Olaf = {Trait.Frost, Trait.Hunter}, 4
    Poppy = {Trait.Witchcraft, Trait.Bastion}, 1
    Rakan = {Trait.Faerie, Trait.Preserver}, 4
    Rumble = {Trait.Sugarcraft, Trait.Blaster, Trait.Vanguard}, 2
    Ryze = {Trait.Portal, Trait.Scholar}, 4
    Seraphine = {Trait.Faerie, Trait.Mage}, 1
    Shen = {Trait.Pyro, Trait.Bastion}, 3
    Shyvana = {Trait.Dragon, Trait.Shapeshifter}, 2
    Smolder = {Trait.Dragon, Trait.Blaster}, 5
    Soraka = {Trait.Sugarcraft, Trait.Mage}, 1
    Swain = {Trait.Frost, Trait.Shapeshifter}, 3
    Syndra = {Trait.Eldritch, Trait.Incantor}, 2
    Tahm_kench = {Trait.Arcana, Trait.Vanguard}, 4
    Taric = {Trait.Portal, Trait.Bastion}, 4
    Tristana = {Trait.Faerie, Trait.Blaster}, 2
    Twitch = {Trait.Frost, Trait.Hunter}, 1
    Varus = {Trait.Pyro, Trait.Blaster}, 4
    Veigar = {Trait.Honeymancy, Trait.Mage}, 3
    Vex = {Trait.Chrono, Trait.Mage}, 3
    Warwick = {Trait.Frost, Trait.Vanguard}, 1
    Wukong = {Trait.Druid}, 3
    Xerath = {Trait.Arcana, Trait.Ascendant}, 5
    Ziggs = {Trait.Honeymancy, Trait.Incantor}, 1
    Zilean = {Trait.Chrono, Trait.Frost, Trait.Preserver}, 2
    Zoe = {Trait.Portal, Trait.Witchcraft, Trait.Scholar}, 1

    def __str__(self):
        return f"{self.cost}-{self.name}"

    def __repr__(self):
        return str(self)

    def __eq__(self, other: object):
        if not isinstance(other, Champion):
            return NotImplemented
        return self.name == other.name

    def __hash__(self):
        return hash(self.name)

    def __gt__(self, other: object):
        if not isinstance(other, Champion):
            return NotImplemented
        if self.cost == other.cost:
            return self.name > other.name
        return self.cost > other.cost

    # def __lt__(self, other: object):
    #     if not isinstance(other, Champion):
    #         return NotImplemented
    #     if self.cost == other.cost:
    #         return self.name < other.name
    #     return self.cost < other.cost


ChampionName: TypeAlias = Literal[
    "Ahri",
    "Akali",
    "Ashe",
    "Bard",
    "Blitzcrank",
    "Briar",
    "Camille",
    "Cassiopeia",
    "Diana",
    "Elise",
    "Ezreal",
    "Fiora",
    "Galio",
    "Gwen",
    "Hecarim",
    "Hwei",
    "Jax",
    "Jayce",
    "Jinx",
    "Kalista",
    "Karma",
    "Kassadin",
    "Katarina",
    "KogMaw",
    "Lillia",
    "Milio",
    "Mordekaiser",
    "Morgana",
    "Nami",
    "Nasus",
    "Neeko",
    "Nilah",
    "Nomsy",
    "Norra",
    "Nunu",
    "Olaf",
    "Poppy",
    "Rakan",
    "Rumble",
    "Ryze",
    "Seraphine",
    "Shen",
    "Shyvana",
    "Smolder",
    "Soraka",
    "Swain",
    "Syndra",
    "Tahm_kench",
    "Taric",
    "Tristana",
    "Twitch",
    "Varus",
    "Veigar",
    "Vex",
    "Warwick",
    "Wukong",
    "Xerath",
    "Ziggs",
    "Zilean",
    "Zoe",
]

champion_names: list[ChampionName] = list(typing.get_args(ChampionName))
