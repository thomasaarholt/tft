from collections import Counter
from dataclasses import field, dataclass
from itertools import combinations
from tft.champions import Champion, ChampionName
from tft.traits import ActiveTrait, Trait, TraitName
from tft.solutions import Solution


@dataclass
class Team:
    champions: list[Champion] = field(default_factory=list)
    emblems: list[Trait] = field(default_factory=list)

    @classmethod
    def from_names(
        cls, champions: list[ChampionName] = [], emblems: list[TraitName] = []
    ):
        """Construct a team from champion names as strings."""
        return Team(
            champions=[Champion[champion] for champion in champions],
            emblems=[Trait[emblem] for emblem in emblems],
        )

    def traits(self):
        champion_traits = [trait for champ in self.champions for trait in champ.traits]
        return champion_traits + self.emblems

    def active_traits(self):
        traits = self.traits()
        counts = Counter(traits)
        active: list[ActiveTrait] = []
        for trait in set(traits):
            active_class = ActiveTrait.new(trait, counts[trait])
            if active_class:
                active.append(active_class)
        return active

    def non_unique_traits(self):
        return [trait for trait in self.active_traits() if trait.level != 1]

    async def find_champs(self, level: int = 7):
        """Find champions that will satisfy Trait Tracker at a given player level."""
        number_champs_to_pick = level - len(self.champions)
        remaining_champs = set(Champion) - set(self.champions)
        print("RUNNING FIND CHAMPS")
        # print(f"{number_champs_to_pick=}, {remaining_champs=}")
        for champ_combination in combinations(remaining_champs, number_champs_to_pick):
            new_team = Team(
                self.champions + list(champ_combination), emblems=self.emblems
            )
            non_unique_active_traits = new_team.non_unique_traits()

            if len(non_unique_active_traits) >= 7:
                missing_champs = set(new_team.champions) - set(self.champions)
                yield Solution(
                    set(new_team.champions), missing_champs, non_unique_active_traits
                )
