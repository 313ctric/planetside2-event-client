use serde::Deserialize;

use std::num::ParseIntError;
use std::str::FromStr;

use crate::census::CensusClient;

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Experience(pub u32);
impl FromStr for Experience {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Experience, Self::Err> {
        Ok(Experience(u32::from_str(s)?))
    }
}
impl Experience {
    pub async fn info(&self, client: &mut CensusClient) -> Option<ExperienceInfo> {
        if let Some(name) = client.experience_name_from_id(*self).await.ok() {
            Some(ExperienceInfo {
                id: *self,
                name: name,
            })
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExperienceInfo {
    id: Experience,
    pub name: String,
}
impl ExperienceInfo {
    pub fn is_squad(&self) -> bool {
        // contains "Squad"
        self.name.contains("Squad")
    }

    pub fn is_spawn(&self) -> bool {
        // contains "Spawn", but not Kill
        self.name.contains("Spawn") && !self.name.contains("Kill")
    }

    pub fn is_kill(&self) -> bool {
        // fairly sure that the actual kill events always fire when you kill someone, and other special versions fire as well
        if self.id == Experience(1) || self.id == Experience(29) {
            // player kill, max kill
            true
        } else if Experience(146) <= self.id && self.id <= Experience(155) {
            // specific kills
            true
        } else {
            false
        }
    }
    pub fn is_assist(&self) -> bool {
        self.id == Experience(2)
    }

    pub fn is_control_point(&self) -> bool {
        match self.id.0 {
            15 => true,  // Control Point - Defend
            16 => true,  // Control Point - Attack
            272 => true, // Convert Capture Point
            _ => false,
        }
    }

    pub fn is_revive(&self) -> bool {
        match self.id.0 {
            7 => true,  // Revive
            53 => true, // Squad Revive
            _ => false,
        }
    }
    pub fn is_heal(&self) -> bool {
        match self.id.0 {
            4 => true,  // Heal Player
            5 => true,  // Heal Assist
            51 => true, // Squad Heal
            _ => false,
        }
    }
    pub fn is_shield_repair(&self) -> bool {
        match self.id.0 {
            438 => true, // Shield Repair
            439 => true, // Squad Shield Repair
            _ => false,
        }
    }

    pub fn is_repair(&self) -> bool {
        // contains "Repair", but is not shield repair
        if self.is_shield_repair() {
            return false;
        }
        self.name.contains("Repair")
    }
    pub fn is_resupply_player(&self) -> bool {
        match self.id.0 {
            34 => true, // Resupply Player
            55 => true, // Squad Resupply
            _ => false,
        }
    }
    pub fn is_resupply_vehicle(&self) -> bool {
        match self.id.0 {
            240 => true, // Vehicle Resupply
            241 => true, // Squad Vehicle Resupply
            _ => false,
        }
    }
    pub fn is_resupply(&self) -> bool {
        self.is_resupply_player() || self.is_resupply_vehicle()
    }

    pub fn is_spot(&self) -> bool {
        // contains "Spot"
        self.name.contains("Spot")
    }
    pub fn is_motion_detect(&self) -> bool {
        match self.id.0 {
            293 => true, // Motion Detect
            294 => true, // Squad Motion Spot
            _ => false,
        }
    }
    pub fn is_radar(&self) -> bool {
        match self.id.0 {
            353 => true, // Scout Radar Detect
            354 => true, // Squad Scout Radar Detect
            _ => false,
        }
    }
    /// if the experience is a spot bonus, motion detect bonus or a radar bonus
    pub fn is_recon(&self) -> bool {
        self.is_spot() || self.is_motion_detect() || self.is_radar()
    }

    pub fn is_vehicle_damage(&self) -> bool {
        // contains "Spot"
        self.name.contains("Damage")
    }

    pub fn is_support(&self) -> bool {
        self.is_spawn()
            || self.is_revive()
            || self.is_heal()
            || self.is_shield_repair()
            || self.is_repair()
            || self.is_resupply()
            || self.is_recon()
    }
}
