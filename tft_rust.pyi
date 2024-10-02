from enum import Enum
from typing import Self

class ChampionRust(Enum):
    Ahri = ...
    Akali = ...
    Ashe = ...
    Bard = ...
    Blitzcrank = ...
    Briar = ...
    Camille = ...
    Cassiopeia = ...
    Diana = ...
    Elise = ...
    Ezreal = ...
    Fiora = ...
    Galio = ...
    Gwen = ...
    Hecarim = ...
    Hwei = ...
    Jax = ...
    Jayce = ...
    Jinx = ...
    Kalista = ...
    Karma = ...
    Kassadin = ...
    Katarina = ...
    KogMaw = ...
    Lillia = ...
    Milio = ...
    Mordekaiser = ...
    Morgana = ...
    Nami = ...
    Nasus = ...
    Neeko = ...
    Nilah = ...
    Nomsy = ...
    Norra = ...
    Nunu = ...
    Olaf = ...
    Poppy = ...
    Rakan = ...
    Rumble = ...
    Ryze = ...
    Seraphine = ...
    Shen = ...
    Shyvana = ...
    Smolder = ...
    Soraka = ...
    Swain = ...
    Syndra = ...
    TahmKench = ...
    Taric = ...
    Tristana = ...
    Twitch = ...
    Varus = ...
    Veigar = ...
    Vex = ...
    Warwick = ...
    Wukong = ...
    Xerath = ...
    Ziggs = ...
    Zilean = ...
    Zoe = ...


class TeamRust:
    def __init__(self, names: list[ChampionRust]) -> None: ...
    @classmethod
    def from_names_py(cls, names: list[str]) -> Self: ...
    def find_solutions_py(self, level: int) -> SolutionIteratorRust: ...
    def traits(self) -> list[TraitRust]: ...

class TraitRust(Enum):
    Ascendant = ...
    Bastion = ...
    BatQueen = ...
    BestFriends = ...
    Blaster = ...
    Hunter = ...
    Incantor = ...
    Mage = ...
    Multistriker = ...
    Preserver = ...
    Scholar = ...
    Shapeshifter = ...
    Vanguard = ...
    Warrior = ...
    Arcana = ...
    Chrono = ...
    Dragon = ...
    Druid = ...
    Eldritch = ...
    Faerie = ...
    Frost = ...
    Honeymancy = ...
    Portal = ...
    Pyro = ...
    Ravenous = ...
    Sugarcraft = ...
    Witchcraft = ...

class ActiveTraitRust:
    @property
    def trait_(self) -> TraitRust: ...
    @property
    def level(self) -> int: ...

class SolutionRust:
    @property
    def champions(self) -> set[ChampionRust]: ...
    @property
    def missing_champions(self) -> set[ChampionRust]: ...
    @property
    def traits(self) -> list[ActiveTraitRust]: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


class SolutionIteratorRust:
    def __init__(self, team: TeamRust, level: int) -> None: ...
    def __iter__(self) -> 'SolutionIteratorRust': ...
    def __next__(self) -> SolutionRust: ...

