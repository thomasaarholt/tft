from tft.team import Team
from tqdm.auto import tqdm


if __name__ == "__main__":
    level = 7
    team = Team.from_names(
        champions=["Ahri", "Zoe", "Rumble", "Rakan"],
        emblems=["Portal", "Bastion"],
    )
    for i, solution in enumerate(team.find_champs(level=level)):
        champs = sorted(solution.missing_champions)
        tqdm.write(f"{i}: {champs} {solution.traits}")
