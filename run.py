from tft.team import Team
import asyncio
async def main():
    level = 8
    team = Team.from_names(
        champions=["Kalista", "Jax", "Kassadin", "Bard" , "Rumble", "TahmKench"],
        # emblems=["Portal", "Bastion"],
    )
    print("Rust:")
    async for solution in team.find_champs_rust(level=level):
        print(solution)

    # print("\n\nPython:")
    # for solution in team.find_champs(level=level):
    #     print(solution)
    
if __name__ == "__main__":
    asyncio.run(main())
