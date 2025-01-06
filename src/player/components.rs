use bevy::prelude::*;

// Define the components for the player entity

// Component for the player's animated sprite
#[derive(Component)]
pub struct AnimatedSprite {
    // Add fields as necessary for the animated sprite
}

// Component for the player's health
#[derive(Component)]
pub struct Health {
    current: i32,
    max: i32,
}

impl Health {
    pub fn new(initial: i32) -> Self {
        Health {
            current: initial,
            max: initial
        }
    }
}

// Enum for different types of guns
pub enum GunType {
    Pistol,
    Rifle,
    Shotgun,
    Sniper
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
            reserve_size
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
            GunType::Sniper => Ammo::new(1, 25)
        };

        Gun {
            gun_type,
            ammo
        }
    }
}
