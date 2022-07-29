use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::{
    census::CensusClient,
    data::{BattleRank, Faction, Timestamp},
};

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Character(pub u64);
impl FromStr for Character {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Character, Self::Err> {
        Ok(Character(u64::from_str(s)?))
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Iterator for Character {
    // we will be counting with usize
    type Item = Character;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        Some(*self)
    }
}
impl Character {
    pub async fn from_name(name: String, client: &mut CensusClient) -> Option<Self> {
        client.character_from_name(name).await.ok()
    }

    pub async fn info(&self, client: &mut CensusClient) -> Option<CharacterInfo> {
        client.character_info_from_id(*self).await.ok()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CharacterInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    pub name: CharacterName,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub faction_id: Faction,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub head_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub title_id: u32,
    pub times: CharacterTimes,
    pub certs: CharacterCerts,
    pub battle_rank: CharacterBattleRank,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub profile_id: u32,
    pub daily_ribbon: CharacterDailyRibbon,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub prestige_level: u32,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CharacterName {
    pub first: String,
    pub first_lower: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CharacterTimes {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub creation: Timestamp,
    pub creation_date: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub last_save: Timestamp,
    pub last_save_date: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub last_login: Timestamp,
    pub last_login_date: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub login_count: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub minutes_played: u32,
}
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct CharacterCerts {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub earned_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub gifted_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub spent_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub available_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub percent_to_next: f32,
}
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct CharacterBattleRank {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub percent_to_next: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: BattleRank,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CharacterDailyRibbon {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub count: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub time: Timestamp,
    pub date: String,
}
