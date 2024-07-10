use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BspInfo {
    pub message: String,
    pub size: u32,
    pub category: String,
    pub entity_count: EntityCount,
    pub intermissions: Vec<Intermission>,
    pub race_routes: Vec<RaceRoute>,
}

impl BspInfo {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_bytes(&std::fs::read(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let entities = bspparser::entities_as_hashmaps(bytes)?;

        let mut info = BspInfo {
            size: bytes.len() as u32,
            ..Default::default()
        };
        let mut e: EntityCount = Default::default();

        for entity in entities.iter() {
            if let Some(classname) = entity.get("classname") {
                let sf = entity.get("spawnflags");

                match classname.as_str() {
                    // misc
                    "worldspawn" => info.message = get_string_value(entity, "message"),

                    // spawns
                    "info_player_coop" => e.spawns.coop += 1,
                    "info_player_deathmatch" => e.spawns.deathmatch += 1,
                    "info_player_start" => e.spawns.start += 1,
                    "info_player_team1" => e.spawns.team1 += 1,
                    "info_player_team2" => e.spawns.team2 += 1,
                    "info_player_team1_deathmatch" => e.spawns.team1_deathmatch += 1,
                    "info_player_team2_deathmatch" => e.spawns.team2_deathmatch += 1,
                    "info_player_teamspawn" | "i_p_t" => e.spawns.teamspawn += 1,

                    // monsters
                    "monster_army" => e.monsters.grunt += 1,
                    "monster_boss" => e.monsters.chton += 1,
                    "monster_demon1" => e.monsters.fiend += 1,
                    "monster_dog" => e.monsters.rottweiler += 1,
                    "monster_enforcer" => e.monsters.enforcer += 1,
                    "monster_fish" => e.monsters.rotfish += 1,
                    "monster_hell_knight" => e.monsters.death_knight += 1,
                    "monster_knight" => e.monsters.knight += 1,
                    "monster_ogre" => e.monsters.ogre += 1,
                    "monster_oldone" => e.monsters.shub_niggurath += 1,
                    "monster_shalrath" => e.monsters.vore += 1,
                    "monster_shambler" => e.monsters.shambler += 1,
                    "monster_tarbaby" => e.monsters.spawn += 1,
                    "monster_wizard" => e.monsters.scrag += 1,
                    "monster_zombie" => e.monsters.zombie += 1,

                    // armors
                    "item_armor1" => e.armors.green_armor += 1,
                    "item_armor2" => e.armors.yellow_armor += 1,
                    "item_armorInv" => e.armors.red_armor += 1,

                    // powerups
                    "item_artifact_envirosuit" => e.powerups.biosuit += 1,
                    "item_artifact_super_damage" => e.powerups.quad += 1,
                    "item_artifact_invulnerability" => e.powerups.pent += 1,
                    "item_artifact_invisibility" => e.powerups.ring += 1,

                    // healthpacks
                    "item_health" if sf.is_none() => e.healthpacks.health_small += 1,
                    "item_health" if sf.is_some_and(|s| s == "1") => {
                        e.healthpacks.health_large += 1
                    }
                    "item_health" if sf.is_some_and(|s| s == "2") => e.healthpacks.megahealth += 1,

                    // misc items
                    "item_key1" => e.items.silver_key += 1,
                    "item_key2" => e.items.gold_key += 1,
                    "item_flag_team1" => e.items.red_flag += 1,
                    "item_flag_team2" => e.items.blue_flag += 1,
                    "item_tfgoal" => e.items.tf_goal += 1,

                    // points
                    "point_start" => e.points.start += 1,
                    "point_zip" => e.points.zip += 1,

                    // ammo
                    "item_shells" if sf.is_none() => e.ammo.shells_small += 1,
                    "item_shells" if sf.is_some() => e.ammo.shells_large += 1,
                    "item_spikes" if sf.is_none() => e.ammo.nails_small += 1,
                    "item_spikes" if sf.is_some() => e.ammo.nails_large += 1,
                    "item_rockets" if sf.is_none() => e.ammo.rockets_small += 1,
                    "item_rockets" if sf.is_some() => e.ammo.rockets_large += 1,
                    "item_cells" if sf.is_none() => e.ammo.cells_small += 1,
                    "item_cells" if sf.is_some() => e.ammo.cells_large += 1,

                    // weapons
                    "weapon_supershotgun" => e.weapons.super_shotgun += 1,
                    "weapon_nailgun" => e.weapons.nailgun += 1,
                    "weapon_supernailgun" => e.weapons.super_nailgun += 1,
                    "weapon_grenadelauncher" => e.weapons.grenade_launcher += 1,
                    "weapon_rocketlauncher" => e.weapons.rocket_launcher += 1,
                    "weapon_lightning" => e.weapons.ligthning_gun += 1,

                    // triggers
                    "trigger_changelevel" => e.triggers.changelevel += 1,
                    "trigger_secret" => e.triggers.secret += 1,
                    "trigger_teleport" => e.triggers.teleport += 1,

                    // race routes
                    "race_route_start" => info.race_routes.push(RaceRoute {
                        name: get_string_value(entity, "race_route_name"),
                        description: get_string_value(entity, "race_route_description"),
                    }),
                    "info_intermission" => info.intermissions.push(Intermission {
                        origin: get_string_value(entity, "origin"),
                        mangle: get_string_value(entity, "mangle"),
                    }),
                    _ => {}
                }
            }
        }

        info.entity_count = e;
        info.category = get_category(&info);

        Ok(info)
    }
}

fn get_string_value(ent: &HashMap<String, String>, key: &str) -> String {
    ent.get(key).map_or("".to_string(), |v| v.to_string())
}

fn get_category(info: &BspInfo) -> String {
    let e = info.entity_count.clone();
    let has_dm_items = !e.ammo.is_empty() || !e.armors.is_empty() || !e.weapons.is_empty();

    if !e.monsters.is_empty() && e.triggers.changelevel > 0 {
        "Single Player".to_string()
    } else if (e.spawns.team1 + e.spawns.team2 + e.items.red_flag + e.items.blue_flag) > 0 {
        "Capture the Flag".to_string()
    } else if e.items.tf_goal > 0 {
        "Team Fortress".to_string()
    } else if info.message.to_lowercase().contains(" trick ")
        || !has_dm_items && e.triggers.teleport >= e.spawns.deathmatch
    {
        "Trick".to_string()
    } else if e.spawns.deathmatch > 0 && e.weapons.is_empty() {
        match info.race_routes.is_empty() && e.points.is_empty() {
            true => "Arena".to_string(),
            false => "Race".to_string(),
        }
    } else if e.spawns.deathmatch > 1 {
        "Deathmatch".to_string()
    } else {
        "Other".to_string()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct EntityCount {
    pub ammo: Ammo,
    pub armors: Armors,
    pub healthpacks: Healthpacks,
    pub items: Items,
    pub monsters: Monsters,
    pub points: Points,
    pub powerups: Powerups,
    pub spawns: Spawns,
    pub triggers: Triggers,
    pub weapons: Weapons,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ammo {
    pub shells_small: u32,
    pub shells_large: u32,
    pub nails_small: u32,
    pub nails_large: u32,
    pub rockets_small: u32,
    pub rockets_large: u32,
    pub cells_small: u32,
    pub cells_large: u32,
}

impl Ammo {
    pub fn count(&self) -> u32 {
        self.shells_small
            + self.shells_large
            + self.nails_small
            + self.nails_large
            + self.rockets_small
            + self.rockets_large
            + self.cells_small
            + self.cells_large
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Armors {
    pub green_armor: u32,
    pub yellow_armor: u32,
    pub red_armor: u32,
}

impl Armors {
    pub fn count(&self) -> u32 {
        self.green_armor + self.yellow_armor + self.red_armor
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Healthpacks {
    pub health_small: u32,
    pub health_large: u32,
    pub megahealth: u32,
}

impl Healthpacks {
    pub fn count(&self) -> u32 {
        self.health_small + self.health_large + self.megahealth
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Items {
    pub silver_key: u32,
    pub gold_key: u32,
    pub red_flag: u32,
    pub blue_flag: u32,
    pub tf_goal: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Monsters {
    pub chton: u32,
    pub death_knight: u32,
    pub enforcer: u32,
    pub fiend: u32,
    pub grunt: u32,
    pub knight: u32,
    pub ogre: u32,
    pub rotfish: u32,
    pub rottweiler: u32,
    pub scrag: u32,
    pub shambler: u32,
    pub shub_niggurath: u32,
    pub spawn: u32,
    pub vore: u32,
    pub zombie: u32,
}
impl Monsters {
    pub fn count(&self) -> u32 {
        self.chton
            + self.death_knight
            + self.enforcer
            + self.fiend
            + self.grunt
            + self.knight
            + self.ogre
            + self.rotfish
            + self.rottweiler
            + self.scrag
            + self.shambler
            + self.shub_niggurath
            + self.spawn
            + self.vore
            + self.zombie
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Powerups {
    pub biosuit: u32,
    pub quad: u32,
    pub pent: u32,
    pub ring: u32,
}

impl Powerups {
    pub fn count(&self) -> u32 {
        self.biosuit + self.quad + self.pent + self.ring
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Spawns {
    pub coop: u32,
    pub deathmatch: u32,
    pub start: u32,
    pub team1: u32,
    pub team1_deathmatch: u32,
    pub team2: u32,
    pub team2_deathmatch: u32,
    pub teamspawn: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Triggers {
    pub changelevel: u32,
    pub secret: u32,
    pub teleport: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Points {
    pub zip: u32,
    pub start: u32,
}

impl Points {
    pub fn count(&self) -> u32 {
        self.zip + self.start
    }
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Weapons {
    pub super_shotgun: u32,
    pub nailgun: u32,
    pub super_nailgun: u32,
    pub grenade_launcher: u32,
    pub rocket_launcher: u32,
    pub ligthning_gun: u32,
}

impl Weapons {
    pub fn count(&self) -> u32 {
        self.super_shotgun
            + self.nailgun
            + self.super_nailgun
            + self.grenade_launcher
            + self.rocket_launcher
            + self.ligthning_gun
    }

    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Intermission {
    pub origin: String,
    pub mangle: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RaceRoute {
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_from_bytes() -> Result<()> {
        let bytes = std::fs::read("tests/files/povdmm4.bsp")?;
        let info = BspInfo::from_bytes(&bytes)?;
        let mut expect = BspInfo {
            message: "DMM4 Arena\\nBy Povo-Hat (http://povo-hat.besmella-quake.com)\\n".to_string(),
            size: 130920,
            category: "Arena".to_string(),
            ..Default::default()
        };
        expect.entity_count.spawns.start = 1;
        expect.entity_count.spawns.deathmatch = 4;
        expect.entity_count.armors.yellow_armor = 2;
        assert_eq!(info, expect);
        Ok(())
    }

    #[test]
    fn test_from_file() -> Result<()> {
        {
            let info = BspInfo::from_file("tests/files/dm3_gpl.bsp")?;
            let expect = BspInfo {
                message: "The Abandoned Base".to_string(),
                size: 1361880,
                category: "Deathmatch".to_string(),
                entity_count: EntityCount {
                    armors: Armors {
                        green_armor: 0,
                        yellow_armor: 1,
                        red_armor: 1,
                    },
                    ammo: Ammo {
                        shells_small: 5,
                        shells_large: 4,
                        nails_small: 7,
                        nails_large: 4,
                        rockets_small: 4,
                        rockets_large: 3,
                        cells_small: 2,
                        cells_large: 1,
                    },
                    healthpacks: Healthpacks {
                        health_small: 4,
                        health_large: 3,
                        megahealth: 3,
                    },
                    triggers: Triggers {
                        changelevel: 1,
                        secret: 0,
                        teleport: 2,
                    },
                    powerups: Powerups {
                        biosuit: 0,
                        quad: 1,
                        pent: 1,
                        ring: 1,
                    },
                    spawns: Spawns {
                        coop: 0,
                        deathmatch: 6,
                        start: 1,
                        team1: 0,
                        team2: 0,
                        team1_deathmatch: 0,
                        team2_deathmatch: 0,
                        teamspawn: 0,
                    },
                    weapons: Weapons {
                        super_shotgun: 1,
                        nailgun: 1,
                        super_nailgun: 1,
                        grenade_launcher: 1,
                        rocket_launcher: 1,
                        ligthning_gun: 1,
                    },
                    ..Default::default()
                },
                race_routes: vec![],
                intermissions: vec![
                    Intermission {
                        origin: "-272 -800 336".to_string(),
                        mangle: "20 45 0".to_string(),
                    },
                    Intermission {
                        origin: "352 -296 -192".to_string(),
                        mangle: "-20 45 0".to_string(),
                    },
                    Intermission {
                        origin: "-736 376 240".to_string(),
                        mangle: "20 30 0".to_string(),
                    },
                    Intermission {
                        origin: "1840 256 64".to_string(),
                        mangle: "20 240 0".to_string(),
                    },
                ],
            };
            assert_eq!(info, expect);
        }
        {
            let info = BspInfo::from_file("tests/files/race17_sdcup.bsp")?;
            let mut expect = BspInfo {
                message: "anubis in hurry by anni (Apr 2021)".to_string(),
                size: 1846812,
                category: "Race".to_string(),
                ..Default::default()
            };
            expect.entity_count.spawns.start = 1;
            expect.entity_count.spawns.deathmatch = 5;
            expect.entity_count.triggers.teleport = 1;
            expect.race_routes = vec![RaceRoute {
                name: "anubis in hurry".to_string(),
                description: "Start > Finish".to_string(),
            }];
            assert_eq!(info, expect);
        }

        Ok(())
    }
}
