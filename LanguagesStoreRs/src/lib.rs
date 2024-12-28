use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

extern crate savefile;
use savefile::prelude::*;

#[macro_use]
extern crate savefile_derive;

// generated at build time
include!(concat!(env!("OUT_DIR"), "/language_config.rs"));
