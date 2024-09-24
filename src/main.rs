pub mod champions;
pub mod team;
pub mod traits;
use team::Team;

fn main() {
    let team = Team::from_names(vec!["Ahri", "TahmKench"]);
    team.find_solutions();
}
