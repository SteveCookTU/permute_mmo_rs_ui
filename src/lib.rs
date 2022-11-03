extern crate core;

mod app;
mod result_widget;

pub use app::*;
use lazy_static::lazy_static;
pub use result_widget::*;

const SPECIES_EN_RAW: &str = include_str!("../resources/species.txt");
const NATURES_EN_RAW: &str = include_str!("../resources/natures.txt");

lazy_static! {
    pub static ref SPECIES_EN: Vec<&'static str> = load_string_list(SPECIES_EN_RAW);
    pub static ref NATURES_EN: Vec<&'static str> = load_string_list(NATURES_EN_RAW);
}

fn load_string_list(list: &str) -> Vec<&str> {
    list.split('\n')
        .map(|s| {
            if s.is_empty() {
                s
            } else if s.as_bytes()[s.len() - 1] == b'\r' {
                &s[..(s.len() - 1)]
            } else {
                s
            }
        })
        .collect()
}
