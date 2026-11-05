use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use semver::{Version, VersionReq};
use std::cmp::Ordering;

#[pyfunction]
fn validate(version: &str) -> bool {
    Version::parse(version).is_ok()
}

#[pyfunction]
fn compare(left: &str, right: &str) -> PyResult<i8> {
    let left = Version::parse(left).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let right = Version::parse(right).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(match left.cmp_precedence(&right) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    })
}

#[pyfunction]
fn matches(version: &str, requirement: &str) -> PyResult<bool> {
    let version = Version::parse(version).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let requirement =
        VersionReq::parse(requirement).map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(requirement.matches(&version))
}

#[pymodule]
fn cargo_semver_python(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(validate, module)?)?;
    module.add_function(wrap_pyfunction!(compare, module)?)?;
    module.add_function(wrap_pyfunction!(matches, module)?)?;
    Ok(())
}
