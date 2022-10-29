use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use pyo3::{pyclass, pymethods};
use rosu_pp::{beatmap::BeatmapAttributes, Beatmap};

#[pyclass(name = "BeatmapAttributes")]
#[derive(Debug)]
pub struct PyBeatmapAttributes {
    #[pyo3(get)]
    ar: f64,
    #[pyo3(get)]
    cs: f64,
    #[pyo3(get)]
    hp: f64,
    #[pyo3(get)]
    od: f64,
    #[pyo3(get)]
    ar_hit_window: f64,
    #[pyo3(get)]
    od_hit_window: f64,
    #[pyo3(get)]
    clock_rate: f64,
    #[pyo3(get)]
    bpm: f64,
    #[pyo3(get)]
    mode: u8,
    #[pyo3(get)]
    version: u8,
    #[pyo3(get)]
    n_circles: u32,
    #[pyo3(get)]
    n_sliders: u32,
    #[pyo3(get)]
    n_spinners: u32,
}

impl PyBeatmapAttributes {
    pub fn new(attrs: BeatmapAttributes, map: &Beatmap) -> Self {
        Self {
            ar: attrs.ar,
            cs: attrs.cs,
            hp: attrs.hp,
            od: attrs.od,
            ar_hit_window: attrs.hit_windows.ar,
            od_hit_window: attrs.hit_windows.od,
            clock_rate: attrs.clock_rate,
            bpm: map.bpm() * attrs.clock_rate,
            mode: map.mode as u8,
            version: map.version,
            n_circles: map.n_circles,
            n_sliders: map.n_sliders,
            n_spinners: map.n_spinners,
        }
    }
}

impl Display for PyBeatmapAttributes {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        <Self as Debug>::fmt(self, f)
    }
}

#[pymethods]
impl PyBeatmapAttributes {
    fn __repr__(&self) -> String {
        self.to_string()
    }
}
