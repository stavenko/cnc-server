use serde::{Serialize, Deserialize};
use nalgebra::{Matrix4, Vector2 };

struct  CoordinationSystem {
  name: String,
  matrix: Matrix4<f64> 
}

pub struct Axis {
  is_homed: bool,
  current_position: f64,
  limit: Vector2<f64>,
  name: String,
}

#[derive(Serialize, Deserialize)]
pub enum SpindleState {
  Off,
  Cw,
  Ccw
}

struct Spindle{
  state: SpindleState,
  speed: f64,
  limit: Vector2<f64>
}

#[derive(Serialize, Deserialize)]
pub struct  Program {
  name: String,
  file: String,
  current_line: String,
}

enum CncMode{
  Preparation,
  Milling
}

struct Machine {
  estop: bool,
  power: bool,
  connected: bool,
}

pub struct CncState {
  mode: CncMode,
  axes: Vec<Axis>,
  spindle: Spindle,
  program_loaded:  Option<Program>,
  machine: Machine,
}
