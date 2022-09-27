//! Utility for generating fish for neural network training data.

use ndarray::Array;
use ndarray::Array3;
use numpy::ToPyArray;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

extern crate fishgen as fishgen_rs;

#[pyclass(name = "fish_output")]
pub struct PyFishOutput {
    pub image: Array3<f32>,
    #[pyo3(get)]
    pub fish_x: f32,
    #[pyo3(get)]
    pub fish_y: f32,
    #[pyo3(get)]
    pub fish_width: f32,
    #[pyo3(get)]
    pub fish_height: f32,
}
#[pymethods]
impl PyFishOutput {
    #[getter]
    pub fn image<'py, 'b>(&self, m: Python<'py>) -> PyResult<&'b numpy::PyArray3<f32>>
    where
        'py: 'b,
    {
        Ok(self.image.to_pyarray(m))
    }
}

impl From<fishgen_rs::FishOutput> for PyFishOutput {
    fn from(fish: fishgen_rs::FishOutput) -> Self {
        let raw_bytes = fish.image.as_raw().iter().map(|v| *v as f32 / 255.0).collect();
        let width = fish.image.width() as usize;
        let height = fish.image.height() as usize;
        let new = Array::from_shape_vec((width, height, 3), raw_bytes).unwrap();

        PyFishOutput {
            image: new.into(),
            fish_x: fish.x as f32 / width as f32,
            fish_y: fish.y as f32 / height as f32,
            fish_width: fish.width as f32 / width as f32,
            fish_height: fish.height as f32 / height as f32,
        }
    }
}

#[pyfunction]
#[pyo3(text_signature = "(height, width, min_scale, max_scale, /)")]
pub fn random_fish(height: u32, width: u32, min_scale: f32, max_scale: f32) -> PyResult<PyFishOutput> {
    let fish = fishgen_rs::random_fish(width, height, min_scale, max_scale);
    let fish = match fish {
        Ok(v) => v,
        Err(e) => return Err(PyRuntimeError::new_err(format!("{e}"))),
    };
    Ok(fish.into())
}

#[pymodule]
fn fishgen(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyFishOutput>()?;
    m.add_function(wrap_pyfunction!(random_fish, m)?)?;
    Ok(())
}
