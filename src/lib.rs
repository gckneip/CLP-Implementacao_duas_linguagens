use pyo3::prelude::*;
use std::thread;
use std::time::Duration;

#[pyfunction]
fn do_long_task() -> PyResult<String> {
    println!("Iniciando tarefa longa em Rust...");
    thread::sleep(Duration::from_secs(3)); 
    println!("Tarefa longa concluída!");
    Ok("Tarefa concluída com sucesso!".to_string())
}


#[pymodule]
fn pintada(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(do_long_task))?;
    Ok(())
}