use serde::Deserialize;

use std::num::ParseIntError;
use std::str::FromStr;

use crate::{
    census::CensusClient,
    data::{Class, Faction},
};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Loadout(pub u32);
impl FromStr for Loadout {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Loadout, Self::Err> {
        Ok(Loadout(u32::from_str(s)?))
    }
}
impl Loadout {
    pub async fn class_info(&self, client: &mut CensusClient) -> Option<ClassInfo> {
        client.class_info_from_loadout(*self).await.ok()
    }
}
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ClassInfo {
    pub loadout_id: Loadout,
    pub faction_id: Faction,
    pub class: Class,
}
