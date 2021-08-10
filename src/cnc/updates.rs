use std::collections::HashMap;
use super::state::Program;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase", tag="t")]
pub enum ProtocolUpdates{
  AxisLimit{name: String, limits: [f64; 2]},
  AxisPosition{positions: HashMap<String, f64>},
  AxisHoming{name: String, is_homed: bool},
  CncState{estop: bool, power: bool, app: bool},
  ProgramList{programs: Vec<String>},
  CurrentProgram{program: Program},
  CurrentProgramExtensions{by_axes: HashMap<String, [f64; 2]>},
  ProgramExecutionLine{line: u64},
  
}

