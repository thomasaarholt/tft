use pyo3::prelude::*;

use crate::{solution::Solution, team::Team};
use crossbeam_channel::{unbounded, Receiver};

#[pyclass(name = "SolutionIteratorRust")]
pub struct SolutionIterator {
    receiver: Receiver<Option<Solution>>,
}

#[pymethods]
impl SolutionIterator {
    #[new]
    pub fn new(team: Team, level: u8) -> Self {
        let (sender, receiver) = unbounded();

        std::thread::spawn(move || {
            for combination in team.generate_combinations(level) {
                if let Some(solution) = team.evaluate_combination(&combination) {
                    if sender.send(Some(solution)).is_err() {
                        break; // The receiver has been dropped
                    }
                }
            }
            let _ = sender.send(None); // Signal completion
        });

        SolutionIterator { receiver }
    }

    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(slf: PyRefMut<Self>) -> PyResult<Option<Py<Solution>>> {
        match slf.receiver.recv() {
            Ok(Some(solution)) => {
                let py_solution = Python::with_gil(|py| Py::new(py, solution))?;
                Ok(Some(py_solution))
            }
            Ok(None) | Err(_) => Ok(None), // No more data or the sender has been dropped
        }
    }
}
