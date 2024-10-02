use crate::{solution::Solution, team::Team};
use crossbeam_channel::{unbounded, Receiver};
use pyo3::prelude::*;

#[pyclass(name = "SolutionIteratorRust")]
pub struct SolutionIterator {
    receiver: Receiver<Option<Solution>>,
}

#[pymethods]
impl SolutionIterator {
    #[new]
    pub fn new(team: Team, level: u8) -> Self {
        let (sender, receiver) = unbounded();

        // Spawn the computation in a new thread
        std::thread::spawn(move || {
            for combination in team.generate_combinations(level) {
                if let Some(solution) = team.evaluate_combination(&combination) {
                    if sender.send(Some(solution)).is_err() {
                        break; // Receiver dropped
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

    fn __next__(slf: PyRefMut<Self>, py: Python) -> PyResult<Option<Py<Solution>>> {
        {
            // Create a scope to limit the lifetime of the borrow
            let receiver = &slf.receiver;

            // Release the GIL during blocking receive
            let result = py.allow_threads(|| receiver.recv());

            match result {
                Ok(Some(solution)) => {
                    // Re-acquire the GIL to create Python objects
                    let py_solution = Py::new(py, solution)?;
                    Ok(Some(py_solution))
                }
                Ok(None) | Err(_) => Ok(None),
            }
        }
    }
}
