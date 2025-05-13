use pyo3::{
    exceptions::PyValueError, pyclass, pymethods, pymodule, types::{PyModule, PyModuleMethods}, Bound, PyErr, PyResult
};

use crate::{BencodeValue, Parser};

#[pyclass]
struct Bencoder {}

#[pymethods]
impl Bencoder {
    #[new]
    fn new() -> Self {
        Bencoder {}
    }

    fn parse(&self, obj: &[u8]) -> Result<BencodeValue, PyErr> {
        Parser::new(obj.to_vec())
            .parse()
            .map_err(|err| PyErr::new::<PyValueError, _>(err.to_string()))
    }
}

#[allow(unused_variables)]
#[pymodule]
pub fn pet(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Bencoder>()?;
    Ok(())
}
