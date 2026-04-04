use pyo3::prelude::*;

#[cfg(feature = "stubgen")]
use pyo3_stub_gen::derive::*;


#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[pyclass]
pub struct {{ cookiecutter.project_name | capitalize }} {

}

#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl {{ cookiecutter.project_name | capitalize }} {

}

#[cfg_attr(feature = "stubgen", gen_stub_pyfunction)]
#[pyfunction]
fn foo(){

}




pub(crate) fn register(_: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(foo, m)?)?;
    m.add_class::<{{ cookiecutter.project_name | capitalize }}>()?;
    Ok(())
}