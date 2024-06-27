use anyhow::Result;

use info::{EntityStats, Info};

pub mod info;

pub fn get_info(data: &[u8]) -> Result<Info> {
    let ents = bspparser::entities_as_hashmaps(data)?;

    let mut info = Info {
        size: data.len() as u32,
        ..Default::default()
    };

    let mut stats: EntityStats = Default::default();

    for ent in ents.iter() {
        if let Some(classname) = ent.get("classname") {
            let sf = ent.get("spawnflags");

            match classname.as_str() {
                "worldspawn" => {
                    info.message = ent.get("message").unwrap_or(&"".to_string()).to_string()
                }
                "info_player_coop" => stats.spawns.coop += 1,
                "info_player_deathmatch" => stats.spawns.deathmatch += 1,
                "info_player_start" => stats.spawns.start += 1,

                "monster_army" => stats.monsters.grunt += 1,
                "monster_boss" => stats.monsters.chton += 1,
                "monster_demon1" => stats.monsters.fiend += 1,
                "monster_dog" => stats.monsters.rottweiler += 1,
                "monster_enforcer" => stats.monsters.enforcer += 1,
                "monster_fish" => stats.monsters.rotfish += 1,
                "monster_hell_knight" => stats.monsters.death_knight += 1,
                "monster_knight" => stats.monsters.knight += 1,
                "monster_ogre" => stats.monsters.ogre += 1,
                "monster_oldone" => stats.monsters.shub_niggurath += 1,
                "monster_shalrath" => stats.monsters.vore += 1,
                "monster_shambler" => stats.monsters.shambler += 1,
                "monster_tarbaby" => stats.monsters.spawn += 1,
                "monster_wizard" => stats.monsters.scrag += 1,
                "monster_zombie" => stats.monsters.zombie += 1,

                "item_armor1" => stats.armors.green_armor += 1,
                "item_armor2" => stats.armors.yellow_armor += 1,
                "item_armorInv" => stats.armors.red_armor += 1,

                "weapon_supershotgun" => stats.weapons.super_shotgun += 1,
                "weapon_nailgun" => stats.weapons.nailgun += 1,
                "weapon_supernailgun" => stats.weapons.super_nailgun += 1,
                "weapon_grenadelauncher" => stats.weapons.grenade_launcher += 1,
                "weapon_rocketlauncher" => stats.weapons.rocket_launcher += 1,
                "weapon_lightning" => stats.weapons.ligthning_gun += 1,

                "item_health" if sf.is_none() => stats.healthpacks.health_small += 1,
                "item_health" if sf.is_some_and(|s| s == "1") => {
                    stats.healthpacks.health_large += 1
                }
                "item_health" if sf.is_some_and(|s| s == "2") => stats.healthpacks.megahealth += 1,

                "item_shells" if sf.is_none() => stats.ammo.shells_small += 1,
                "item_shells" if sf.is_some() => stats.ammo.shells_large += 1,
                "item_spikes" if sf.is_none() => stats.ammo.nails_small += 1,
                "item_spikes" if sf.is_some() => stats.ammo.nails_large += 1,
                "item_rockets" if sf.is_none() => stats.ammo.rockets_small += 1,
                "item_rockets" if sf.is_some() => stats.ammo.rockets_large += 1,
                "item_cells" if sf.is_none() => stats.ammo.cells_small += 1,
                "item_cells" if sf.is_some() => stats.ammo.cells_large += 1,

                "item_key1" => stats.keys.silver += 1,
                "item_key2" => stats.keys.gold += 1,

                "item_artifact_envirosuit" => stats.powerups.biosuit += 1,
                "item_artifact_super_damage" => stats.powerups.pent += 1,
                "item_artifact_invulnerability" => stats.powerups.quad += 1,
                "item_artifact_invisibility" => stats.powerups.ring += 1,

                "trigger_changelevel" => stats.triggers.changelevel += 1,
                "trigger_secret" => stats.triggers.secret += 1,
                "trigger_teleport" => stats.triggers.teleport += 1,
                _ => {}
            }
        }
    }

    info.entity_stats = stats;

    Ok(info)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::info::{Ammo, Armors, Healthpacks, Powerups, Spawns, Triggers, Weapons};

    use super::*;

    #[test]
    fn test_get_info() -> Result<()> {
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
