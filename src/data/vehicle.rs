use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use std::num::ParseIntError;
use std::str::FromStr;

use crate::{census::CensusClient, data::LocaleText};

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vehicle(pub u32);
impl FromStr for Vehicle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Vehicle, Self::Err> {
        Ok(Vehicle(u32::from_str(s)?))
    }
}
impl Vehicle {
    pub async fn info(&self, client: &mut CensusClient) -> Option<VehicleInfo> {
        client.vehicle_info_from_id(*self).await.ok()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct VehicleInfo {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub vehicle_id: Vehicle,
    pub name: LocaleText,
    pub description: LocaleText,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub type_id: u32,
    pub type_name: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub cost_resource_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub image_set_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub image_id: u32,
    pub image_path: String,
}
impl VehicleInfo {
    pub fn is_ground(&self) -> bool {
        match self.type_id {
            2 => true, // hover tank
            5 => true, // 4 wheeled ground vehicle
            _ => false,
        }
    }
    pub fn is_air(&self) -> bool {
        match self.type_id {
            1 => true, // light aircraft
            8 => true, // drop pod
            _ => false,
        }
    }
    pub fn is_boat(&self) -> bool {
        false
    }
}
