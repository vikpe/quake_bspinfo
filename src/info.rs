#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Info {
    pub message: String,
    pub size: u32,
    pub entity_stats: EntityStats,
    pub race_routes: Vec<RaceRoute>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EntityStats {
    pub spawns: Spawns,
    pub func: Func,
    pub monsters: Monsters,
    pub armors: Armors,
    pub weapons: Weapons,
    pub healthpacks: Healthpacks,
    pub ammo: Ammo,
    pub keys: Keys,
    pub powerups: Powerups,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Armors {
    pub green_armor: u32,
    pub yellow_armor: u32,
    pub red_armor: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Func {
    pub changelevel: u32,
    pub secret: u32,
    pub teleport: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Healthpacks {
    pub health_small: u32,
    pub health_large: u32,
    pub megahealth: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Keys {
    pub silver: u32,
    pub gold: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Powerups {
    pub biosuit: u32,
    pub quad: u32,
    pub pent: u32,
    pub ring: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Spawns {
    pub coop: u32,
    pub deathmatch: u32,
    pub start: u32,
    pub start2: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Weapons {
    pub super_shotgun: u32,
    pub nailgun: u32,
    pub super_nailgun: u32,
    pub grenade_launcher: u32,
    pub rocket_launcher: u32,
    pub ligthning_gun: u32,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RaceRoute {
    pub name: String,
    pub description: String,
}
