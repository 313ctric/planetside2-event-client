use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::num::ParseIntError;
use std::str::FromStr;

use std::fmt;

mod character;
pub use character::*;
mod vehicle;
pub use vehicle::*;
mod loadout;
pub use loadout::*;
mod fire_mode;
pub use fire_mode::*;
mod experience;
pub use experience::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocaleText {
    de: String,
    en: String,
    es: String,
    fr: String,
    it: String,
    tr: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Environment {
    PC,
    Ps4US,
    Ps4EU,
}
impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::PC => write!(f, "ps2"),
            Environment::Ps4US => write!(f, "ps2ps4us"),
            Environment::Ps4EU => write!(f, "ps2ps4eu"),
        }
    }
}

#[derive(Serialize_repr, Deserialize, TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum World {
    // All, // this should select all servers, but didn't work in testing
    Connery = 1,
    Miller = 10,
    Cobalt = 13,
    Emerald = 17,
    Jaeger = 19,
    Apex = 24,
    Briggs = 25,
    SolTech = 40,
}
impl FromStr for World {
    type Err = TryFromPrimitiveError<World>;

    fn from_str(s: &str) -> Result<World, Self::Err> {
        let val = u32::from_str(s).expect("tried to deserialize non-numeric world id");
        World::try_from(val)
    }
}
impl World {
    pub const ALL_WORLDS: &'static [World] = &[
        World::Apex,
        World::Briggs,
        World::Cobalt,
        World::Connery,
        World::Emerald,
        World::Jaeger,
        World::Miller,
        World::SolTech,
    ];
}

#[derive(Serialize, Deserialize, TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Zone {
    Indar = 2,
    Hossin = 4,
    Amerish = 6,
    Esamir = 8,
    Oshur = 344,
    VRTrainingNC = 96,
    VRTrainingTR = 97,
    VRTrainingVS = 98,
    Koltyr = 10000,
    Desolation = 20000,
    // Nexus
    Sanctuary = 131434,
}
impl FromStr for Zone {
    type Err = TryFromPrimitiveError<Zone>;

    fn from_str(s: &str) -> Result<Zone, Self::Err> {
        let val = match u32::from_str(s) {
            Ok(x) => x,
            Err(_) => return Err(TryFromPrimitiveError { number: 0 }),
        };
        Zone::try_from(val)
    }
}

#[derive(Serialize, Deserialize, TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Faction {
    None = 0,
    VanuSovereignty = 1,
    NewConglomerate = 2,
    TerranRepublic = 3,
    NSOperatives = 4,
}
impl FromStr for Faction {
    type Err = TryFromPrimitiveError<Faction>;

    fn from_str(s: &str) -> Result<Faction, Self::Err> {
        let val = u8::from_str(s).expect("tried to deserialize non-numeric faction id");
        Faction::try_from(val)
    }
}

#[derive(Deserialize, TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Class {
    Infiltrator = 1,
    LightAssault = 3,
    CombatMedic = 4,
    Engineer = 5,
    HeavyAssault = 6,
    MAX = 7,
}
impl FromStr for Class {
    type Err = TryFromPrimitiveError<Class>;

    fn from_str(s: &str) -> Result<Class, Self::Err> {
        let val = u8::from_str(s).expect("tried to deserialize non-numeric class id");
        Class::try_from(val)
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Timestamp(pub u64);
impl FromStr for Timestamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Timestamp, Self::Err> {
        Ok(Timestamp(u64::from_str(s)?))
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Skill(pub u32);
impl FromStr for Skill {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Skill, Self::Err> {
        Ok(Skill(u32::from_str(s)?))
    }
}
#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Item(pub u32);
impl FromStr for Item {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Item, Self::Err> {
        Ok(Item(u32::from_str(s)?))
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Achievement(pub u32);
impl FromStr for Achievement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Achievement, Self::Err> {
        Ok(Achievement(u32::from_str(s)?))
    }
}
#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct BattleRank(pub u32);
impl FromStr for BattleRank {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<BattleRank, Self::Err> {
        Ok(BattleRank(u32::from_str(s)?))
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Facility(pub u32);
impl FromStr for Facility {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Facility, Self::Err> {
        Ok(Facility(u32::from_str(s)?))
    }
}
/// not actually a specific weapon, more of a general class of weapon
#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Weapon(pub u32);
impl FromStr for Weapon {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Weapon, Self::Err> {
        Ok(Weapon(u32::from_str(s)?))
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Outfit(pub u64);
impl FromStr for Outfit {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Outfit, Self::Err> {
        Ok(Outfit(u64::from_str(s)?))
    }
}
