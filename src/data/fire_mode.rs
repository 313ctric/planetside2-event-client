use serde::Deserialize;

use std::num::ParseIntError;
use std::str::FromStr;

use crate::{
    census::CensusClient,
    data::{Item, LocaleText},
};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FireMode(pub u32);
impl FromStr for FireMode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<FireMode, Self::Err> {
        Ok(FireMode(u32::from_str(s)?))
    }
}
impl FireMode {
    pub async fn info(&self, client: &mut CensusClient) -> Option<FireModeInfo> {
        client.fire_info_from_fire_mode(*self).await.ok()
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct FireModeInfo {
    pub item_id: Item,
    /// "primary" or "secondary"
    pub weapon_type: String,
    pub weapon_name: LocaleText,
    pub weapon_is_vehicle_weapon: bool,
}
