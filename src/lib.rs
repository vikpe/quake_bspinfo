use std::collections::HashMap;

use anyhow::Result;

pub fn get_info(data: &[u8]) -> Result<Info> {
    let ents = bspparser::entities_as_hashmaps(data)?;

    let message = ents
        .iter()
        .find(|hmap| hmap.get("classname").unwrap() == "worldspawn")
        .map(|hmap| hmap.get("message").unwrap_or(&"".to_string()).to_string())
        .unwrap_or("".to_string());

    let info = Info {
        message,
        size: data.len() as u32,
        entity_stats: EntityStats {
            spawns: Spawns {
                coop: _count(&ents, "info_player_coop"),
                deathmatch: _count(&ents, "info_player_deathmatch"),
                start: _count(&ents, "info_player_start"),
                start2: _count(&ents, "info_player_start2"),
            },
            func: Func {
                changelevel: _count(&ents, "trigger_changelevel"),
                secret: _count(&ents, "trigger_secret"),
                teleport: _count(&ents, "trigger_teleport"),
            },
            monsters: Monsters {
                chton: _count(&ents, "monster_boss"),
                death_knight: _count(&ents, "monster_hell_knight"),
                enforcer: _count(&ents, "monster_enforcer"),
                fiend: _count(&ents, "monster_demon1"),
                grunt: _count(&ents, "monster_army"),
                knight: _count(&ents, "monster_demon1"),
                ogre: _count(&ents, "monster_ogre"),
                rotfish: _count(&ents, "monster_dog"),
                rottweiler: _count(&ents, "monster_dog"),
                scrag: _count(&ents, "monster_wizard"),
                shambler: _count(&ents, "monster_shambler"),
                shub_niggurath: _count(&ents, "monster_oldone"),
                spawn: _count(&ents, "monster_tarbaby"),
                vore: _count(&ents, "monster_shalrath"),
                zombie: _count(&ents, "monster_zombie"),
            },
            armors: Armors {
                green_armor: _count(&ents, "item_armor1"),
                yellow_armor: _count(&ents, "item_armor2"),
                red_armor: _count(&ents, "item_armorInv"),
            },
            weapons: Weapons {
                super_shotgun: _count(&ents, "weapon_supershotgun"),
                nailgun: _count(&ents, "weapon_nailgun"),
                super_nailgun: _count(&ents, "weapon_supernailgun"),
                grenade_launcher: _count(&ents, "weapon_grenadelauncher"),
                rocket_launcher: _count(&ents, "weapon_rocketlauncher"),
                ligthning_gun: _count(&ents, "weapon_lightning"),
            },
            healthpacks: Healthpacks {
                health_small: _count_sf(&ents, "item_health", None),
                health_large: _count_sf(&ents, "item_health", Some("1".to_string())),
                megahealth: _count_sf(&ents, "item_health", Some("2".to_string())),
            },
            ammo: Ammo {
                shells_small: _count_sf(&ents, "item_shells", None),
                shells_large: _count_sf(&ents, "item_shells", Some("1".to_string())),
                nails_small: _count_sf(&ents, "item_spikes", None),
                nails_large: _count_sf(&ents, "item_spikes", Some("1".to_string())),
                rockets_small: _count_sf(&ents, "item_rockets", None),
                rockets_large: _count_sf(&ents, "item_rockets", Some("1".to_string())),
                cells_small: _count_sf(&ents, "item_cells", None),
                cells_large: _count_sf(&ents, "item_cells", Some("1".to_string())),
            },
            keys: Keys {
                silver: _count(&ents, "item_key1"),
                gold: _count(&ents, "item_key2"),
            },
            powerups: Powerups {
                biosuit: _count(&ents, "item_artifact_envirosuit"),
                pent: _count(&ents, "item_artifact_super_damage"),
                quad: _count(&ents, "item_artifact_invulnerability"),
                ring: _count(&ents, "item_artifact_invisibility"),
            },
            runes: Runes {
                haste: 0,
                regeneration: 0,
                resistance: 0,
                strength: 0,
            },
        },
        race_routes: vec![],
    };

    Ok(info)
}

fn _count(entities: &[HashMap<String, String>], classname: &str) -> u32 {
    entities
        .iter()
        .filter(|hmap| hmap.get("classname").is_some_and(|c| c == classname))
        .count() as u32
}

fn _count_sf(
    entities: &[HashMap<String, String>],
    classname: &str,
    spawnflag: Option<String>,
) -> u32 {
    entities
        .iter()
        .filter(|hmap| {
            hmap.get("classname").is_some_and(|c| c == classname)
                && hmap.get("spawnflags") == spawnflag.as_ref()
        })
        .count() as u32
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Info {
    message: String,
    size: u32,
    entity_stats: EntityStats,
    race_routes: Vec<RaceRoute>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntityStats {
    spawns: Spawns,
    func: Func,
    monsters: Monsters,
    armors: Armors,
    weapons: Weapons,
    healthpacks: Healthpacks,
    ammo: Ammo,
    keys: Keys,
    powerups: Powerups,
    runes: Runes,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ammo {
    shells_small: u32,
    shells_large: u32,
    nails_small: u32,
    nails_large: u32,
    rockets_small: u32,
    rockets_large: u32,
    cells_small: u32,
    cells_large: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Armors {
    green_armor: u32,
    yellow_armor: u32,
    red_armor: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Func {
    changelevel: u32,
    secret: u32,
    teleport: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Healthpacks {
    health_small: u32,
    health_large: u32,
    megahealth: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Keys {
    silver: u32,
    gold: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Monsters {
    chton: u32,
    death_knight: u32,
    enforcer: u32,
    fiend: u32,
    grunt: u32,
    knight: u32,
    ogre: u32,
    rotfish: u32,
    rottweiler: u32,
    scrag: u32,
    shambler: u32,
    shub_niggurath: u32,
    spawn: u32,
    vore: u32,
    zombie: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Powerups {
    biosuit: u32,
    quad: u32,
    pent: u32,
    ring: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Runes {
    haste: u32,
    regeneration: u32,
    resistance: u32,
    strength: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spawns {
    coop: u32,
    deathmatch: u32,
    start: u32,
    start2: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Weapons {
    super_shotgun: u32,
    nailgun: u32,
    super_nailgun: u32,
    grenade_launcher: u32,
    rocket_launcher: u32,
    ligthning_gun: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaceRoute {
    name: String,
    description: String,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_default() -> Result<()> {
        {
            let info = get_info(&fs::read("tests/files/povdmm4.bsp")?)?;
            let mut expect = Info {
                message: "DMM4 Arena\\nBy Povo-Hat (http://povo-hat.besmella-quake.com)\\n"
                    .to_string(),
                size: 130920,
                ..Default::default()
            };
            expect.entity_stats.spawns.start = 1;
            expect.entity_stats.spawns.deathmatch = 4;
            expect.entity_stats.armors.yellow_armor = 2;
            assert_eq!(info, expect);
        }
        {
            let info = get_info(&fs::read("tests/files/dm3_gpl.bsp")?)?;
            let expect = Info {
                message: "The Abandoned Base".to_string(),
                size: 1361880,
                entity_stats: EntityStats {
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
                    func: Func {
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
                        start2: 0,
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
                ..Default::default()
            };
            assert_eq!(info, expect);
        }

        Ok(())
    }
}
