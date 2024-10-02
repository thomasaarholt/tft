import asyncio
from collections.abc import AsyncGenerator, Generator
from collections import Counter
from dataclasses import dataclass, field
from itertools import combinations

from tft.champions import Champion, ChampionName
from tft.solution import Solution
from tft.solution_iterator import AsyncSolutionIterator
from tft.traits import ActiveTrait, Trait, TraitName

from tft_rust import TeamRust


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
        return {trait for trait in self.active_traits() if trait.level != 1}

    def _generate_combinations(
        self, level: int, is_async: bool
    ) -> Generator[tuple[Champion, ...], None, None]:
        """Helper function to generate combinations of champions."""
        number_champs_to_pick = level - len(self.champions)
        remaining_champs = set(Champion) - set(self.champions)
        generator = combinations(remaining_champs, number_champs_to_pick)
        for champ_combination in generator:
            yield champ_combination

    def _evaluate_combination(
        self, champ_combination: tuple[Champion, ...]
    ) -> Solution | None:
        """Helper function to evaluate a champion combination and return a Solution if valid."""
        new_team = Team(self.champions + list(champ_combination), emblems=self.emblems)
        non_unique_active_traits = new_team.non_unique_traits()

        if len(non_unique_active_traits) >= 7:
            # We remove Wukong because for the purpose of trait tracker, he is useless
            missing_champs = (
                set(new_team.champions) - set(self.champions) - set([Champion.Wukong])
            )
            return Solution(
                set(new_team.champions),
                missing_champs,
                non_unique_active_traits,
            )
        return None

    def find_champs(self, level: int = 7) -> Generator[Solution, None, None]:
        """Find champions that will satisfy Trait Tracker at a given player level."""
        for champ_combination in self._generate_combinations(level, False):
            solution = self._evaluate_combination(champ_combination)
            if solution:
                yield solution

    async def async_find_champs(self, level: int = 7) -> AsyncGenerator[Solution, None]:
        """Asynchronous version of find_champs."""

        try:
            for champ_combination in self._generate_combinations(level, True):
                await asyncio.sleep(0)  # Yield control to the event loop
                if solution := self._evaluate_combination(champ_combination):
                    yield solution
        except asyncio.CancelledError:
            print("find_champs generator was cancelled.")
            raise  # Re-raise to allow proper cancellation flow

    def find_champs_rust(self, level: int = 7) -> AsyncSolutionIterator:
        """Asynchronous version of find_champs."""
        names = [champ.name for champ in self.champions]
        team = TeamRust.from_names_py(names=names)
        return AsyncSolutionIterator(team.find_solutions_py(level))
