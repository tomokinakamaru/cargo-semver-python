use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use semver::{Version, VersionReq};
use std::cmp::Ordering;

#[pyfunction]
fn parse<'py>(py: Python<'py>, version: &str) -> PyResult<Bound<'py, PyDict>> {
    let version = Version::parse(version).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let result = PyDict::new(py);
    let prerelease = match version.pre.as_str() {
        "" => None,
        value => Some(value),
    };
    let build = match version.build.as_str() {
        "" => None,
        value => Some(value),
    };
    result.set_item("major", version.major)?;
    result.set_item("minor", version.minor)?;
    result.set_item("patch", version.patch)?;
    result.set_item("prerelease", prerelease)?;
    result.set_item("build", build)?;
    Ok(result)
}

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
    module.add_function(wrap_pyfunction!(parse, module)?)?;
    module.add_function(wrap_pyfunction!(validate, module)?)?;
    module.add_function(wrap_pyfunction!(compare, module)?)?;
    module.add_function(wrap_pyfunction!(matches, module)?)?;
    Ok(())
}
