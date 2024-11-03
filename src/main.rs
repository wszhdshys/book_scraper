#![windows_subsystem = "windows"]
#![feature(tuple_trait)]
mod functions;

use crate::functions::gui::crate_gui;
use std::error::Error;

const MY_FONTS_BYTES: &[u8] = include_bytes!("../font/loli.ttf");

fn main() -> Result<(), Box<dyn Error>> {
    crate_gui()?;
    Ok(())
}
