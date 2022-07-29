use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::{deserialize_bool_from_anything, deserialize_number_from_string};

use crate::data::{
    Achievement, BattleRank, Character, Experience, Facility, Faction, FireMode, Item, Loadout,
    Outfit, Skill, Timestamp, Vehicle, Weapon, World, Zone,
};

// LOTS of issues with strange zone_id numbers

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EventType {
    #[serde(rename = "all")]
    All,

    AchievementEarned,
    BattleRankUp,
    Death,
    ItemAdded,
    SkillAdded,
    VehicleDestroy,
    GainExperience,

    PlayerFacilityCapture,
    PlayerFacilityDefend,

    // ignore characters field, trigger for the whole world (server)
    ContinentLock,
    ContinentUnlock,
    FacilityControl,
    MetagameEvent,

    PlayerLogin,
    PlayerLogout,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "event_name")]
pub enum EventPayload {
    AchievementEarned(AchievementEvent),
    BattleRankUp(BattleRankEvent),
    Death(DeathEvent),
    ItemAdded(ItemAddEvent),
    SkillAdded(SkillAddEvent),
    VehicleDestroy(VehicleDestroyEvent),
    GainExperience(ExperienceEvent),

    PlayerFacilityCapture(PlayerFacilityEvent),
    PlayerFacilityDefend(PlayerFacilityEvent),

    // ignore characters field, trigger for the whole world (server)
    ContinentLock(ContinentEvent),
    ContinentUnlock(ContinentEvent),
    FacilityControl(FacilityControlEvent),
    MetagameEvent(MetagameEvent),

    PlayerLogin(PlayerLogEvent),
    PlayerLogout(PlayerLogEvent),
}

// TODO: some of these may be numbers
#[derive(Deserialize, Debug, Clone)]
pub struct ContinentEvent {
    // continent lock/unlock
    pub event_type: String,
    pub metagame_event_id: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub previous_faction: Faction,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub triggering_faction: Faction,

    pub vs_population: String,
    pub nc_population: String,
    pub tr_population: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AchievementEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub achievement_id: Achievement,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct BattleRankEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub battle_rank: BattleRank,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

/// Player login/logout
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PlayerLogEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct FacilityControlEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub facility_id: Facility,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub old_faction_id: Faction,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub new_faction_id: Faction,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub outfit_id: Outfit,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

// TODO: some of these may be numbers
#[derive(Deserialize, Debug, Clone)]
pub struct MetagameEvent {
    pub metagame_event_id: String,
    pub metagame_event_state: String,

    pub experience_bonus: String,

    pub faction_nc: String,
    pub faction_tr: String,
    pub faction_vs: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemAddEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,

    pub context: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub item_id: Item,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub item_count: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SkillAddEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub skill_id: Skill,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

/// Player facility capture/defend
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PlayerFacilityEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub facility_id: Facility,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub outfit_id: Outfit,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ExperienceEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub other_id: Character,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub experience_id: Experience,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub amount: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub loadout_id: Loadout,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct DeathEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_fire_mode_id: FireMode,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_loadout_id: Loadout,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_vehicle_id: Vehicle,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_weapon_id: Weapon,

    /// the character that has just died
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_loadout_id: Loadout,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_headshot: bool,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct VehicleDestroyEvent {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_loadout_id: Loadout,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_vehicle_id: Vehicle,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub attacker_weapon_id: Weapon,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub character_id: Character,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub vehicle_id: Vehicle,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub facility_id: Facility,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub faction_id: Faction,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: World,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub zone_id: Zone,
}
