use serde::{Serialize, Deserialize};
use super::state;

#[derive(Serialize, Deserialize)]
#[serde(tag="t")]
pub enum Command {
  Lcnc(bool),
  Estop(bool),
  Power(bool),
  LoadProgram{name: String},
  ListPrograms,
  CreateProgram{name: String},
  UpdateProgram{name: String, content: String},
  DeleteProgram{name: String},
  RunCurrentProgram{from_line: u64},
  GetCurrentProgramExtensions,
  SetSpindleSpeed{dir: state::SpindleState, speed: f64}

}
