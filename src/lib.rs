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
                coop: count_classname(&ents, "info_player_coop"),
                deathmatch: count_classname(&ents, "info_player_deathmatch"),
                start: count_classname(&ents, "info_player_start"),
                start2: count_classname(&ents, "info_player_start2"),
            },
            func: Func {
                changelevel: count_classname(&ents, "trigger_changelevel"),
                secret: count_classname(&ents, "trigger_secret"),
                teleport: count_classname(&ents, "trigger_teleport"),
            },
            monsters: Monsters {
                chton: count_classname(&ents, "monster_boss"),
                death_knight: count_classname(&ents, "monster_hell_knight"),
                enforcer: count_classname(&ents, "monster_enforcer"),
                fiend: count_classname(&ents, "monster_demon1"),
                grunt: count_classname(&ents, "monster_army"),
                knight: count_classname(&ents, "monster_demon1"),
                ogre: count_classname(&ents, "monster_ogre"),
                rotfish: count_classname(&ents, "monster_dog"),
                rottweiler: count_classname(&ents, "monster_dog"),
                scrag: count_classname(&ents, "monster_wizard"),
                shambler: count_classname(&ents, "monster_shambler"),
                shub_niggurath: count_classname(&ents, "monster_oldone"),
                spawn: count_classname(&ents, "monster_tarbaby"),
                vore: count_classname(&ents, "monster_shalrath"),
                zombie: count_classname(&ents, "monster_zombie"),
            },
            armors: Armors {
                green_armor: count_classname(&ents, "item_armor1"),
                yellow_armor: count_classname(&ents, "item_armor2"),
                red_armor: count_classname(&ents, "item_armorInv"),
            },
            weapons: Weapons {
                super_shotgun: count_classname(&ents, "weapon_supershotgun"),
                nailgun: count_classname(&ents, "weapon_nailgun"),
                super_nailgun: count_classname(&ents, "weapon_supernailgun"),
                grenade_launcher: count_classname(&ents, "weapon_grenadelauncher"),
                rocket_launcher: count_classname(&ents, "weapon_rocketlauncher"),
                ligthning_gun: count_classname(&ents, "weapon_lightning"),
            },
            healthpacks: Healthpacks {
                health_small: 0,
                health_large: 0,
                megahealth: 0,
            },
            ammo: Ammo {
                shells_small: 0,
                shells_large: 0,
                nails_small: 0,
                nails_large: 0,
                rockets_small: 0,
                rockets_large: 0,
                cells_small: 0,
                cells_large: 0,
            },
            keys: Keys {
                silver: count_classname(&ents, "item_key1"),
                gold: count_classname(&ents, "item_key2"),
            },
            powerups: Powerups {
                biosuit: count_classname(&ents, "item_artifact_envirosuit"),
                pent: count_classname(&ents, "item_artifact_super_damage"),
                quad: count_classname(&ents, "item_artifact_invulnerability"),
                ring: count_classname(&ents, "item_artifact_invisibility"),
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

fn count_classname(entities: &[HashMap<String, String>], classname: &str) -> u32 {
    entities
        .iter()
        .filter(|hmap| hmap.get("classname").unwrap() == classname)
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
