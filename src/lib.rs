mod jogo;
use pyo3::prelude::*;
use jogo::Jogo;

#[pymodule]
fn pintada(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Jogo>()?;
    Ok(())
}