//! The available magic the gathering formats.
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Format {
    Standard,
    Modern,
    Legacy,
    Vintage,
    Commander,
    Future,
    Pauper,
    PauperCommander,
    Pioneer,
    Penny,
    Duel,
    OldSchool,
    Historic,
    HistoricBrawl,
    Gladiator,
    Brawl,
    Premodern,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Format::*;
        write!(
            f,
            "{}",
            match self {
                Standard => "standard",
                Modern => "modern",
                Legacy => "legacy",
                Vintage => "vintage",
                Commander => "commander",
                Future => "future",
                Pauper => "pauper",
                PauperCommander => "paupercommander",
                Pioneer => "pioneer",
                Penny => "penny",
                Duel => "duel",
                OldSchool => "oldschool",
                Historic => "historic",
                HistoricBrawl => "historicbrawl",
                Gladiator => "gladiator",
                Brawl => "brawl",
                Premodern => "premodern",
            }
        )
    }
}
