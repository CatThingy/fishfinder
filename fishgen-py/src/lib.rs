use ndarray::Array;
use ndarray::Array3;
use pyo3::prelude::*;

#[pyclass]
pub struct PyFishOutput {
    pub image: Array3<f32>,
    pub fish_x: f32,
    pub fish_y: f32,
    pub fish_width: f32,
    pub fish_height: f32,
}

impl From<fishgen::FishOutput> for PyFishOutput {
    fn from(fish: fishgen::FishOutput) -> Self {
        let raw_bytes = fish.image.as_raw().iter().map(|v| *v as f32).collect();
        let width = fish.image.width() as usize;
        let height = fish.image.width() as usize;
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
fn random_fish(height: u32, width: u32) -> PyResult<PyFishOutput> {
    let fish = fishgen::random_fish(width, height);
    Ok(fish.into())
}

#[pymodule]
fn fishgen_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyFishOutput>()?;
    m.add_function(wrap_pyfunction!(random_fish, m)?)?;
    Ok(())
}
