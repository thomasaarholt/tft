from dataclasses import dataclass

from tft.champions import Champion
from tft.traits import ActiveTrait


@dataclass
class Solution:
    champions: set[Champion]
    missing_champions: set[Champion]
    traits: set[ActiveTrait]

    @property
    def cost(self):
        return sum([champ.cost for champ in self.champions])

    @property
    def missing_cost(self):
        return sum([champ.cost for champ in self.missing_champions])

    def __str__(self):
        return ", ".join(map(str, self.missing_champions))
