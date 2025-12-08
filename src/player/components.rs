use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct IsSelected;

// Define the components for the player entity
#[derive(Component)]
pub struct Bot {
    pub class: BotClass,
    pub team_nr: u8,
}

#[derive(Component)]
/// Component for the player's program. A bot with this component will
/// be ready to start moving
pub struct ProgramLoaded;

#[derive(Component)]
/// The spawn place of the bot. Used for respawm purposes
pub struct SpawnPlace(pub Vec3);

#[derive(Component)]
/// Component for the bot's health. It contains the current health,
/// the max health, the last time the bot was damaged, the health
/// regen rate and the sprites for the health bar
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub no_regen_timer: Option<Timer>,
    pub regen_rate: i32,
    pub background_sprite: Sprite,
    pub foreground_sprite: Sprite,
}

#[derive(Component)]
/// The class of the bot, contains information about the bot's health, gun, view distance, resolution and view angle
pub struct BotClass {
    name: String,
    health: Health,
    gun: Gun,
    pub view_distance: f32,
    pub resolution: u8, // Amout of rays cast by the bot
    pub view_angle: f32,
}

impl BotClass {
    /// Creates a baseic bot class,
    /// 120 degrees fov, sees up to 2000.0
    /// medium resolution
    pub fn new_basic() -> Self {
        BotClass {
            name: "Basic".to_string(),
            health: Health::new(100.0),
            gun: Gun::new(GunType::Rifle),
            view_angle: 120.0 * PI / 180.0,
            resolution: 7,
            view_distance: 2000.0,
        }
    }

    /// Creates a bot class representing a sniper,
    /// Tighter field of view but sees far
    /// same resolution as basic
    pub fn new_sniper() -> Self {
        BotClass {
            name: "Sniper".to_string(),
            health: Health::new(75.0),
            gun: Gun::new(GunType::Sniper),
            view_angle: 60.0 * PI / 180.0,
            resolution: 7,
            view_distance: 5000.0,
        }
    }
}

#[derive(Component)]
/// A bot with this component will be considered dead.
/// This component is added when the bot's program crashes
pub struct Crashed;

impl Health {
    pub fn new(initial: f32) -> Self {
        Health {
            current: initial,
            max: initial,
            no_regen_timer: None,
            regen_rate: 5, // Five points per second base rate
            background_sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            },
            foreground_sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            },
        }
    }
}

// Enum for different types of guns
pub enum GunType {
    Pistol,
    Rifle,
    Shotgun,
    Sniper,
}

// Component for the player's ammo
#[derive(Component)]
pub struct Ammo {
    in_magazine: i32,
    out_magazine: i32,
    magazine_size: i32,
    reserve_size: i32,
}

impl Ammo {
    fn new(magazine_size: i32, reserve_size: i32) -> Self {
        Ammo {
            in_magazine: magazine_size,
            out_magazine: reserve_size,
            magazine_size,
            reserve_size,
        }
    }
}

// Component for the player's gun
#[derive(Component)]
pub struct Gun {
    gun_type: GunType,
    ammo: Ammo,
}

impl Gun {
    pub fn new(gun_type: GunType) -> Self {
        let ammo = match &gun_type {
            GunType::Pistol => Ammo::new(12, 48),
            GunType::Rifle => Ammo::new(50, 150),
            GunType::Shotgun => Ammo::new(2, 18),
            GunType::Sniper => Ammo::new(1, 25),
        };

        Gun { gun_type, ammo }
    }
}
