from tft.team import Team

if __name__ == "__main__":
    level = 8
    team = Team.from_names(
        champions=["Kalista", "Jax", "Kassadin", "Bard" , "Rumble", "TahmKench"],
        # emblems=["Portal", "Bastion"],
    )
    print("Rust:")
    for solution in team.find_champs_rust(level=level):
        print(solution)

    # print("\n\nPython:")
    # for solution in team.find_champs(level=level):
    #     print(solution)
