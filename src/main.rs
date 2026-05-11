mod app;
mod sky;
mod star;
mod spectral_type;

use crate::app::App;
use color_eyre::eyre::Result;

const WIDTH: u16 = 10000;
const HEIGHT: u16 = 10000;
const NUM_OF_STARS: usize = 1000000;
const CONSTELLATION_SIZE: u16 = 10;
const STAR_PALETTE: [char; 10] = ['·', '◦', '✧', '☆', '*', '⋆', '✦', '✶', '★', '✮'];

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))
}