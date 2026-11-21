mod prepare;
pub(crate) use prepare::prepare;

mod render;
pub(crate) use render::render;

mod utils;
use utils::{area, center, width};