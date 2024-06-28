# quake_bspinfo

> Extract Quake related information from .bsp files

```rust
let data = fs::read("dm3.mvd")?;
let info: BspInfo = quake_bspinfo::parse(&data)?;

// result
struct BspInfo {
    message: String,
    size: u32,
    sha256: String,
    entity_count: EntityCount,
    intermissions: Vec<Intermission>,
    race_routes: Vec<RaceRoute>,
}

struct EntityCount {
    ammo: Ammo,
    armors: Armors,
    healthpacks: Healthpacks,
    items: Items,
    monsters: Monsters,
    powerups: Powerups,
    spawns: Spawns,
    triggers: Triggers,
    weapons: Weapons,
}

struct Ammo {
    shells_small: u32,
    shells_large: u32,
    nails_small: u32,
    nails_large: u32,
    rockets_small: u32,
    rockets_large: u32,
    cells_small: u32,
    cells_large: u32,
}

struct Armors {
    green_armor: u32,
    yellow_armor: u32,
    red_armor: u32,
}

struct Healthpacks {
    health_small: u32,
    health_large: u32,
    megahealth: u32,
}

struct Items {
    silver_key: u32,
    gold_key: u32,
    red_flag: u32,
    blue_flag: u32,
    tf_goal: u32,
}

struct Monsters {
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

struct Powerups {
    biosuit: u32,
    quad: u32,
    pent: u32,
    ring: u32,
}

struct Spawns {
    coop: u32,
    deathmatch: u32,
    start: u32,
    team1: u32,
    team1_deathmatch: u32,
    team2: u32,
    team2_deathmatch: u32,
    teamspawn: u32,
}

struct Triggers {
    changelevel: u32,
    secret: u32,
    teleport: u32,
}

struct Weapons {
    super_shotgun: u32,
    nailgun: u32,
    super_nailgun: u32,
    grenade_launcher: u32,
    rocket_launcher: u32,
    ligthning_gun: u32,
}

struct Intermission {
    origin: String,
    mangle: String,
}

struct RaceRoute {
    name: String,
    description: String,
}
```
