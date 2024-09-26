use std::collections::HashSet;

use pyo3::prelude::*;
use rayon::prelude::*;

use crate::{champions::Champion, solution::Solution, team::Team};

#[pyclass]
pub struct SolutionIterator {
    team: Team,
    combinations: Vec<HashSet<Champion>>,
    index: usize,
    buffer: Vec<Solution>,
}

#[pymethods]
impl SolutionIterator {
    #[new]
    pub fn new(team: Team, level: u8) -> Self {
        let combinations = team.generate_combinations(level);
        SolutionIterator {
            team,
            combinations,
            index: 0,
            buffer: Vec::new(),
        }
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<Solution>>> {
        loop {
            if let Some(solution) = slf.buffer.pop() {
                let py_solution = Py::new(py, solution)?;
                return Ok(Some(py_solution));
            }
            if slf.index >= slf.combinations.len() {
                return Ok(None);
            }

            // Calculate the range for the batch
            let end = (slf.index + 1000).min(slf.combinations.len());
            let start = slf.index;

            // Update slf.index before borrowing slf.combinations
            slf.index = end;

            // Now borrow slf.combinations
            let batch = &slf.combinations[start..end];

            let team_clone = slf.team.clone();

            let mut solutions: Vec<Solution> = batch
                .par_iter()
                .filter_map(|combination| team_clone.evaluate_combination(combination))
                .collect();

            solutions.reverse(); // Reverse to pop from the end

            slf.buffer = solutions;
        }
    }
}
