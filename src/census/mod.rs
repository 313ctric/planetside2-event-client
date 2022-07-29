const CENSUS_BASE_URL: &str = "http://census.daybreakgames.com/s:{service_id}/json/get/ps2:v2/";

use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use reqwest::Client;
use serde_json::Value;

use crate::data::{
    Character, CharacterInfo, Class, ClassInfo, Experience, Faction, FireMode, FireModeInfo, Item,
    Loadout, LocaleText, Vehicle, VehicleInfo,
};

use serde::Deserialize;

#[derive(Clone)]
pub struct CensusClient {
    base_url: String,
    client: Client,

    vehicle_cache: HashMap<Vehicle, VehicleInfo>,
    experience_cache: HashMap<Experience, String>,
    loadout_cache: HashMap<Loadout, ClassInfo>,
    /// a cache for mapping character names to ids
    character_cache: HashMap<String, Character>,
    fire_mode_cache: HashMap<FireMode, FireModeInfo>,
}
impl CensusClient {
    pub fn new(service_id: String) -> Self {
        CensusClient {
            base_url: CENSUS_BASE_URL
                .to_owned()
                .replace("{service_id}", &service_id),
            client: Client::new(),

            vehicle_cache: HashMap::new(),
            experience_cache: HashMap::new(),
            loadout_cache: HashMap::new(),
            character_cache: HashMap::new(),
            fire_mode_cache: HashMap::new(),
        }
    }

    // performs a request to the api
    async fn get(client: &mut Client, query: String) -> Result<Value, Box<dyn Error>> {
        let resp = client.get(query).send().await?.text().await?;
        match serde_json::from_str::<Value>(&resp) {
            Ok(x) => Ok(x),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// fetches a character's id from their name
    pub async fn character_from_name(
        &mut self,
        character_name: String,
    ) -> Result<Character, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct CharacterContainer {
            character_name_list: Vec<Value>,
            returned: u32,
        }
        let name = character_name.to_lowercase();
        let err_str = "error parsing census character name response";
        match self.character_cache.get(&name) {
            Some(x) => Ok(*x),
            None => {
                let url = format!("{}character_name?name.first_lower={}", self.base_url, name);
                let resp = CensusClient::get(&mut self.client, url).await?;
                let character = serde_json::from_value::<CharacterContainer>(resp)?
                    .character_name_list
                    .pop()
                    .ok_or(err_str)?;
                let id = Character {
                    0: u64::from_str(character["character_id"].as_str().ok_or(err_str)?)?,
                };
                self.character_cache.insert(name, id);
                Ok(id)
            }
        }
    }

    /// fetches information about a character from the character's id
    pub async fn character_info_from_id(
        &mut self,
        character_id: Character,
    ) -> Result<CharacterInfo, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct CharacterInfoContainer {
            character_list: Vec<CharacterInfo>,
            returned: u32,
        }
        let err_str = "error parsing census character response response";
        let url = format!("{}character?character_id={}", self.base_url, character_id.0);
        let resp = CensusClient::get(&mut self.client, url).await?;
        let character_info = serde_json::from_value::<CharacterInfoContainer>(resp)?
            .character_list
            .pop()
            .ok_or(err_str)?;
        Ok(character_info)
    }

    /// fetches the weapon name, type and if it is on a vehicle from a fire mode id
    pub async fn fire_info_from_fire_mode(
        &mut self,
        fire_mode_id: FireMode,
    ) -> Result<FireModeInfo, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct FireModeContainer {
            fire_mode_list: Vec<Value>,
            returned: u32,
        }
        let err_str = "error parsing census fire mode response";
        match self.fire_mode_cache.get(&fire_mode_id) {
            Some(x) => Ok(x.clone()),
            None => {
                let url = format!("{}fire_mode?c:join=item^inject_at:item_info^show:name'is_vehicle_weapon&fire_mode_id={}", self.base_url, fire_mode_id.0);
                let resp = CensusClient::get(&mut self.client, url).await?;
                let info = serde_json::from_value::<FireModeContainer>(resp)?
                    .fire_mode_list
                    .pop()
                    .ok_or(err_str)?;
                let fire_info = FireModeInfo {
                    item_id: Item::from_str(info["item_id"].as_str().ok_or(err_str)?)?,
                    weapon_type: info["type"].as_str().ok_or(err_str)?.to_owned(),
                    weapon_name: serde_json::from_value::<LocaleText>(
                        info["item_info"].as_object().ok_or(err_str)?["name"].clone(),
                    )?,
                    weapon_is_vehicle_weapon: info["item_info"].as_object().ok_or(err_str)?
                        ["is_vehicle_weapon"]
                        .as_str()
                        .ok_or(err_str)?
                        != "0",
                };
                self.fire_mode_cache.insert(fire_mode_id, fire_info.clone());
                Ok(fire_info)
            }
        }
    }

    /// fetches the information about a vehicle from it's id
    pub async fn vehicle_info_from_id(
        &mut self,
        vehicle_id: Vehicle,
    ) -> Result<VehicleInfo, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct VehicleInfoContainer {
            vehicle_list: Vec<VehicleInfo>,
            returned: u32,
        }

        let err_str = "error parsing census vehicle response";
        match self.vehicle_cache.get(&vehicle_id) {
            Some(x) => Ok(x.clone()),
            None => {
                let url = format!("{}vehicle?vehicle_id={}", self.base_url, vehicle_id.0);
                let resp = CensusClient::get(&mut self.client, url).await?;
                let vehicle = serde_json::from_value::<VehicleInfoContainer>(resp)?
                    .vehicle_list
                    .pop()
                    .ok_or(err_str)?;
                self.vehicle_cache.insert(vehicle_id, vehicle.clone());
                Ok(vehicle)
            }
        }
    }

    /// fetches the name of a given type of experience event from it's id
    pub async fn experience_name_from_id(
        &mut self,
        experience_id: Experience,
    ) -> Result<String, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct ExperienceContainer {
            experience_list: Vec<Value>,
            returned: u32,
        }
        let err_str = "error parsing census experience name";
        match self.experience_cache.get(&experience_id) {
            Some(x) => Ok(x.clone()),
            None => {
                let url = format!(
                    "{}experience?experience_id={}",
                    self.base_url, experience_id.0
                );
                let resp = CensusClient::get(&mut self.client, url).await?;
                let exp_val = serde_json::from_value::<ExperienceContainer>(resp)?
                    .experience_list
                    .pop()
                    .ok_or(err_str)?;
                let name = exp_val["description"].as_str().ok_or(err_str)?;
                self.experience_cache.insert(experience_id, name.to_owned());
                Ok(name.to_owned())
            }
        }
    }

    /// fetches the class and faction of a player from a loadout id
    pub async fn class_info_from_loadout(
        &mut self,
        loadout_id: Loadout,
    ) -> Result<ClassInfo, Box<dyn Error>> {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct LoadoutContainer {
            loadout_list: Vec<Value>,
            returned: u32,
        }
        let err_str = "error parsing census loadout response";
        match self.loadout_cache.get(&loadout_id) {
            Some(x) => Ok(x.clone()),
            None => {
                let url = format!(
                    "{}loadout?c:join=profile^inject_at:class^show:profile_type_id&loadout_id={}",
                    self.base_url, loadout_id.0
                );
                let resp = CensusClient::get(&mut self.client, url).await?;
                let loadout = serde_json::from_value::<LoadoutContainer>(resp)?
                    .loadout_list
                    .pop()
                    .ok_or(err_str)?;
                let player = ClassInfo {
                    loadout_id: loadout_id,
                    faction_id: Faction::from_str(loadout["faction_id"].as_str().ok_or(err_str)?)?,
                    class: Class::from_str(
                        loadout["class"].as_object().ok_or(err_str)?["profile_type_id"]
                            .as_str()
                            .ok_or(err_str)?,
                    )?,
                };
                self.loadout_cache.insert(loadout_id, player);
                Ok(player)
            }
        }
    }
}
