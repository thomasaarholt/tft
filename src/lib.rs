use champions::Champion;
use pyo3::prelude::*;
pub mod champions;
pub mod solution;
pub mod solution_iterator;
pub mod team;
pub mod traits;
use solution::Solution;
use solution_iterator::SolutionIterator;
use team::Team;
use traits::{ActiveTrait, Trait};

#[pyfunction]
fn find_solutions(level: u8, names: Vec<String>) -> SolutionIterator {
    let team = Team::from_names(names);
    SolutionIterator::new(team, level)
}

#[pymodule]
fn tft_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_solutions, m)?)?;
    m.add_class::<Champion>()?;
    m.add_class::<Team>()?;
    m.add_class::<Trait>()?;
    m.add_class::<ActiveTrait>()?;
    m.add_class::<Solution>()?;
    m.add_class::<SolutionIterator>()?;
    Ok(())
}
