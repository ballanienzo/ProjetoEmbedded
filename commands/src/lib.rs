#![no_std]

use serde::{Deserialize, Serialize};

pub const BAUD_RATE: u32 = 115200;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    PlayNote(u32),
    StopSound,
    Sleep(usize),
}
