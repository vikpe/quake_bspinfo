use std::collections::HashMap;

use anyhow::Result;

use crate::info::{EntityCount, Info, RaceRoute};

pub fn parse(data: &[u8]) -> Result<Info> {
    let entities = bspparser::entities_as_hashmaps(data)?;

    let mut info = Info {
        size: data.len() as u32,
        ..Default::default()
    };
    let mut e: EntityCount = Default::default();

    for entity in entities.iter() {
        if let Some(classname) = entity.get("classname") {
            let sf = entity.get("spawnflags");

            match classname.as_str() {
                // misc
                "worldspawn" => {
                    info.message = entity
                        .get("message")
                        .map_or("".to_string(), |m| m.to_string());
                }

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
                "item_artifact_super_damage" => e.powerups.pent += 1,
                "item_artifact_invulnerability" => e.powerups.quad += 1,
                "item_artifact_invisibility" => e.powerups.ring += 1,

                // healthpacks
                "item_health" if sf.is_none() => e.healthpacks.health_small += 1,
                "item_health" if sf.is_some_and(|s| s == "1") => e.healthpacks.health_large += 1,
                "item_health" if sf.is_some_and(|s| s == "2") => e.healthpacks.megahealth += 1,

                // misc items
                "item_key1" => e.items.silver_key += 1,
                "item_key2" => e.items.gold_key += 1,
                "item_flag_team1" => e.items.red_flag += 1,
                "item_flag_team2" => e.items.blue_flag += 1,
                "item_tfgoal" => e.items.tf_goal += 1,

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
                "race_route_start" => {
                    let name = opt_or_empty(entity, "race_route_name");
                    let description = opt_or_empty(entity, "race_route_description");
                    info.race_routes.push(RaceRoute { name, description })
                }
                _ => {}
            }
        }
    }

    info.entity_count = e;

    Ok(info)
}

fn opt_or_empty(ent: &HashMap<String, String>, key: &str) -> String {
    ent.get(key).map(|v| v.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::info::{
        Ammo, Armors, EntityCount, Healthpacks, Info, Powerups, RaceRoute, Spawns, Triggers,
        Weapons,
    };
    use crate::parse::parse;

    #[test]
    fn test_parse() -> Result<()> {
        {
            let info = parse(&fs::read("tests/files/povdmm4.bsp")?)?;
            let mut expect = Info {
                message: "DMM4 Arena\\nBy Povo-Hat (http://povo-hat.besmella-quake.com)\\n"
                    .to_string(),
                size: 130920,
                ..Default::default()
            };
            expect.entity_count.spawns.start = 1;
            expect.entity_count.spawns.deathmatch = 4;
            expect.entity_count.armors.yellow_armor = 2;
            assert_eq!(info, expect);
        }
        {
            let info = parse(&fs::read("tests/files/dm3_gpl.bsp")?)?;
            let expect = Info {
                message: "The Abandoned Base".to_string(),
                size: 1361880,
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
                ..Default::default()
            };
            assert_eq!(info, expect);
        }
        {
            let info = parse(&fs::read("tests/files/race17_sdcup.bsp")?)?;
            let mut expect = Info {
                message: "anubis in hurry by anni (Apr 2021)".to_string(),
                size: 1846812,
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
