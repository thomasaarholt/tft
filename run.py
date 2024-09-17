from tft.team import Team
from tqdm.auto import tqdm


def main():
    level = 6
    team = Team.from_names(
        champions=[
            # "Ahri",
            # "Zoe"
        ],
        emblems=[
            # "Portal",
            # "Bastion"
        ],
    )
    i = 0
    for solution in team.find_champs(level=level):
        i += 1
        tqdm.write(
            f"{i}: {[(champ.name, champ.cost) for champ in sorted(solution.missing_champions)]} {solution.traits}"
        )


if __name__ == "__main__":
    main()
