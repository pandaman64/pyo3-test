extern crate pyo3;

use pyo3::prelude::*;

#[test]
fn call_many_times() -> PyResult<()> {
    let guard = Python::acquire_gil();
    let py = guard.python();
    let math = py.import("math")?;
    let exp = math.get("exp")?;

    for i in 0..100000 {
        println!("{}", i);
        exp.call1((1.0_f64, ))?;
    }
    
    Ok(())
}

