//! Core data structures for the binary map format.

use bevy::prelude::{Asset, Vec2};
use serde::{Deserialize, Serialize};

/// Current version of the map format.
pub const MAP_FORMAT_VERSION: u32 = 1;

/// File extension for AFG map files.
pub const MAP_FILE_EXTENSION: &str = "afg.map";

// ============== MAIN MAP STRUCTURE ==============

/// Root structure for a game map.
#[derive(Debug, Clone, Serialize, Deserialize, Asset, bevy::reflect::TypePath)]
pub struct GameMap {
    /// Format version for compatibility checking.
    pub version: u32,
    /// Map metadata (title, author, size, etc.).
    pub metadata: MapMetadata,
    /// Ordered list of layers (rendered back to front).
    pub layers: Vec<Layer>,
    /// Special gameplay zones.
    pub zones: Vec<Zone>,
    /// Custom textures embedded in the file.
    pub embedded_assets: Vec<EmbeddedAsset>,
    /// Optional preview thumbnail for map browser.
    pub thumbnail: Option<Thumbnail>,
}

impl GameMap {
    /// Creates a new empty map with the given title and size.
    pub fn new(title: impl Into<String>, width: f32, height: f32) -> Self {
        Self {
            version: MAP_FORMAT_VERSION,
            metadata: MapMetadata {
                title: title.into(),
                author: None,
                size: Vec2::new(width, height),
                background_color: [30, 30, 40, 255], // Dark blue-gray
            },
            layers: vec![
                Layer::new("Background", LayerType::Background),
                Layer::new("Terrain", LayerType::Terrain),
                Layer::new("Props", LayerType::Props),
                Layer::new("Foreground", LayerType::Foreground),
            ],
            zones: Vec::new(),
            embedded_assets: Vec::new(),
            thumbnail: None,
        }
    }

    /// Serializes the map to binary format.
    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::serde::encode_to_vec(self, bincode::config::standard())
    }

    /// Deserializes a map from binary format.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::serde::decode_from_slice(bytes, bincode::config::standard()).map(|(map, _)| map)
    }

    /// Returns the layer with the given type, if it exists.
    pub fn get_layer(&self, layer_type: LayerType) -> Option<&Layer> {
        self.layers.iter().find(|l| l.layer_type == layer_type)
    }

    /// Returns a mutable reference to the layer with the given type.
    pub fn get_layer_mut(&mut self, layer_type: LayerType) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.layer_type == layer_type)
    }

    /// Adds an embedded asset and returns its ID.
    pub fn add_embedded_asset(&mut self, name: String, format: ImageFormat, data: Vec<u8>) -> u32 {
        let id = self.embedded_assets.len() as u32;
        self.embedded_assets.push(EmbeddedAsset {
            id,
            name,
            format,
            data,
        });
        id
    }

    /// Gets an embedded asset by ID.
    pub fn get_embedded_asset(&self, id: u32) -> Option<&EmbeddedAsset> {
        self.embedded_assets.iter().find(|a| a.id == id)
    }
}

impl Default for GameMap {
    fn default() -> Self {
        Self::new("Untitled Map", 1000.0, 1000.0)
    }
}

// ============== METADATA ==============

/// Map metadata and global settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMetadata {
    /// Display name of the map.
    pub title: String,
    /// Optional author name.
    pub author: Option<String>,
    /// Map dimensions in world units.
    pub size: Vec2,
    /// Background color as RGBA.
    pub background_color: [u8; 4],
}

/// Preview thumbnail for the map browser.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// PNG image data.
    pub data: Vec<u8>,
}

// ============== LAYERS ==============

/// A layer containing map objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    /// Display name of the layer.
    pub name: String,
    /// Layer type determines z-order and default behavior.
    pub layer_type: LayerType,
    /// Whether the layer is visible in-game.
    pub visible: bool,
    /// Objects in this layer.
    pub objects: Vec<MapObject>,
}

impl Layer {
    /// Creates a new empty layer.
    pub fn new(name: impl Into<String>, layer_type: LayerType) -> Self {
        Self {
            name: name.into(),
            layer_type,
            visible: true,
            objects: Vec::new(),
        }
    }

    /// Returns the z-index for this layer type.
    pub fn z_index(&self) -> f32 {
        self.layer_type.z_index()
    }
}

/// Types of layers with predefined z-ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerType {
    /// Rendered behind everything (z = -100).
    Background,
    /// Main gameplay layer with collision (z = 0).
    Terrain,
    /// Decorative objects (z = 50).
    Props,
    /// Rendered above players (z = 100).
    Foreground,
}

impl LayerType {
    /// Returns the z-index for rendering.
    pub fn z_index(&self) -> f32 {
        match self {
            LayerType::Background => -100.0,
            LayerType::Terrain => 0.0,
            LayerType::Props => 50.0,
            LayerType::Foreground => 100.0,
        }
    }

    /// Returns whether this layer type typically has collision.
    pub fn has_collision_by_default(&self) -> bool {
        matches!(self, LayerType::Terrain)
    }
}

// ============== OBJECTS ==============

/// A visual/physical object in the map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    /// Unique identifier for editor reference.
    pub id: u64,
    /// Position, rotation, and scale.
    pub transform: Transform2D,
    /// Visual shape of the object.
    pub shape: Shape,
    /// Texture or color for rendering.
    pub texture: TextureRef,
    /// Optional collision shape (None = no collision).
    pub collision: Option<CollisionShape>,
}

impl MapObject {
    /// Creates a new object with the given shape at the origin.
    pub fn new(id: u64, shape: Shape) -> Self {
        Self {
            id,
            transform: Transform2D::default(),
            shape,
            texture: TextureRef::Color([128, 128, 128, 255]),
            collision: None,
        }
    }

    /// Builder method to set position.
    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.transform.position = Vec2::new(x, y);
        self
    }

    /// Builder method to set rotation in radians.
    pub fn with_rotation(mut self, radians: f32) -> Self {
        self.transform.rotation = radians;
        self
    }

    /// Builder method to set texture.
    pub fn with_texture(mut self, texture: TextureRef) -> Self {
        self.texture = texture;
        self
    }

    /// Builder method to enable collision using the visual shape.
    pub fn with_collision(mut self) -> Self {
        self.collision = Some(CollisionShape::SameAsVisual);
        self
    }

    /// Builder method to set a custom collision shape.
    pub fn with_custom_collision(mut self, shape: Shape) -> Self {
        self.collision = Some(CollisionShape::Custom(shape));
        self
    }
}

/// 2D transform data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform2D {
    /// Position in world units.
    pub position: Vec2,
    /// Rotation in radians.
    pub rotation: f32,
    /// Scale factor (1.0 = original size).
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

// ============== SHAPES ==============

/// Shape definitions for visual rendering and collision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Shape {
    /// Axis-aligned rectangle.
    Rectangle {
        width: f32,
        height: f32,
    },
    /// Circle with given radius.
    Circle {
        radius: f32,
    },
    /// Capsule (pill shape) - useful for characters.
    Capsule {
        radius: f32,
        length: f32,
    },
    /// Convex polygon defined by vertices.
    /// Vertices should be in counter-clockwise order.
    /// Non-convex polygons will be auto-decomposed for collision.
    Polygon {
        vertices: Vec<Vec2>,
    },
}

impl Shape {
    /// Creates a rectangle shape.
    pub fn rectangle(width: f32, height: f32) -> Self {
        Shape::Rectangle { width, height }
    }

    /// Creates a circle shape.
    pub fn circle(radius: f32) -> Self {
        Shape::Circle { radius }
    }

    /// Creates a capsule shape.
    pub fn capsule(radius: f32, length: f32) -> Self {
        Shape::Capsule { radius, length }
    }

    /// Creates a polygon shape from vertices.
    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        Shape::Polygon { vertices }
    }

    /// Creates a regular polygon with the given number of sides.
    pub fn regular_polygon(sides: usize, radius: f32) -> Self {
        let vertices = (0..sides)
            .map(|i| {
                let angle = (i as f32) * std::f32::consts::TAU / (sides as f32);
                Vec2::new(angle.cos() * radius, angle.sin() * radius)
            })
            .collect();
        Shape::Polygon { vertices }
    }
}

/// Collision shape configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollisionShape {
    /// Use the same shape as the visual.
    SameAsVisual,
    /// Use a different shape for collision.
    Custom(Shape),
}

// ============== TEXTURES ==============

/// Reference to a texture for rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureRef {
    /// A texture built into the game.
    BuiltIn(BuiltInTexture),
    /// Index into the map's embedded_assets.
    Embedded(u32),
    /// Solid color fill (RGBA).
    Color([u8; 4]),
}

/// Built-in textures bundled with the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuiltInTexture {
    // Walls and terrain
    BrickWall,
    ConcreteWall,
    MetalPlate,
    WoodPlanks,
    Stone,
    Grass,
    Sand,
    Water,

    // Props
    Crate,
    Barrel,
    Sandbag,
    Fence,

    // Special
    Invisible, // For collision-only objects
}

impl BuiltInTexture {
    /// Returns the asset path for this built-in texture.
    pub fn asset_path(&self) -> &'static str {
        match self {
            BuiltInTexture::BrickWall => "textures/walls/brick.png",
            BuiltInTexture::ConcreteWall => "textures/walls/concrete.png",
            BuiltInTexture::MetalPlate => "textures/walls/metal.png",
            BuiltInTexture::WoodPlanks => "textures/walls/wood.png",
            BuiltInTexture::Stone => "textures/terrain/stone.png",
            BuiltInTexture::Grass => "textures/terrain/grass.png",
            BuiltInTexture::Sand => "textures/terrain/sand.png",
            BuiltInTexture::Water => "textures/terrain/water.png",
            BuiltInTexture::Crate => "textures/props/crate.png",
            BuiltInTexture::Barrel => "textures/props/barrel.png",
            BuiltInTexture::Sandbag => "textures/props/sandbag.png",
            BuiltInTexture::Fence => "textures/props/fence.png",
            BuiltInTexture::Invisible => "",
        }
    }
}

// ============== EMBEDDED ASSETS ==============

/// A texture embedded in the map file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedAsset {
    /// Unique identifier within this map.
    pub id: u32,
    /// Display name for the asset.
    pub name: String,
    /// Image format.
    pub format: ImageFormat,
    /// Raw image bytes.
    pub data: Vec<u8>,
}

/// Supported image formats for embedded assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

// ============== ZONES ==============

/// A special gameplay zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Unique identifier for editor reference.
    pub id: u64,
    /// Display name for the zone.
    pub name: String,
    /// Zone boundary shape.
    pub shape: Shape,
    /// Position and rotation.
    pub transform: Transform2D,
    /// Zone behavior.
    pub zone_type: ZoneType,
}

impl Zone {
    /// Creates a new spawn zone.
    pub fn spawn(id: u64, team: u8, shape: Shape, position: Vec2) -> Self {
        Self {
            id,
            name: format!("Team {} Spawn", team),
            shape,
            transform: Transform2D {
                position,
                ..Default::default()
            },
            zone_type: ZoneType::TeamSpawn { team, capacity: 5 },
        }
    }

    /// Creates a new capture point.
    pub fn capture_point(id: u64, name: impl Into<String>, shape: Shape, position: Vec2) -> Self {
        Self {
            id,
            name: name.into(),
            shape,
            transform: Transform2D {
                position,
                ..Default::default()
            },
            zone_type: ZoneType::CapturePoint { capture_time: 10.0 },
        }
    }
}

/// Types of gameplay zones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneType {
    // === Spawns ===
    /// Spawn area for a team.
    TeamSpawn {
        /// Team identifier (0, 1, etc.).
        team: u8,
        /// Maximum bots that can spawn here simultaneously.
        capacity: u8,
    },

    // === Objectives ===
    /// Point that can be captured by standing in it.
    CapturePoint {
        /// Time in seconds to capture.
        capture_time: f32,
    },
    /// Location where a flag spawns.
    FlagSpawn,
    /// Goal area for flag capture (team-specific).
    GoalArea {
        /// Team that scores by bringing flags here.
        team: u8,
    },

    // === Gameplay Modifiers ===
    /// Deals damage over time to entities in the zone.
    DamageZone {
        /// Damage per second.
        damage_per_second: f32,
    },
    /// Heals entities over time.
    HealingZone {
        /// Health restored per second.
        heal_per_second: f32,
    },
    /// Modifies movement speed.
    SpeedModifier {
        /// Multiplier (1.5 = 50% faster, 0.5 = 50% slower).
        multiplier: f32,
    },
    /// Prevents weapons from firing.
    NoFireZone,
    /// Reduces movement speed (convenience alias for SpeedModifier < 1.0).
    SlowZone {
        /// Multiplier (typically 0.3-0.7).
        multiplier: f32,
    },
    /// Blocks line of sight for AI.
    SmokeZone,
}

impl ZoneType {
    /// Returns a display color for editor visualization (RGBA).
    pub fn editor_color(&self) -> [u8; 4] {
        match self {
            ZoneType::TeamSpawn { team, .. } => {
                if *team == 0 {
                    [100, 100, 255, 100] // Blue
                } else {
                    [255, 100, 100, 100] // Red
                }
            }
            ZoneType::CapturePoint { .. } => [255, 255, 100, 100], // Yellow
            ZoneType::FlagSpawn => [255, 200, 50, 100],            // Orange
            ZoneType::GoalArea { .. } => [100, 255, 100, 100],     // Green
            ZoneType::DamageZone { .. } => [255, 50, 50, 100],     // Bright red
            ZoneType::HealingZone { .. } => [50, 255, 50, 100],    // Bright green
            ZoneType::SpeedModifier { multiplier } => {
                if *multiplier > 1.0 {
                    [100, 200, 255, 100] // Cyan (speed boost)
                } else {
                    [150, 100, 50, 100] // Brown (slow)
                }
            }
            ZoneType::NoFireZone => [200, 200, 200, 100], // Gray
            ZoneType::SlowZone { .. } => [150, 100, 50, 100], // Brown
            ZoneType::SmokeZone => [180, 180, 180, 150],  // Light gray
        }
    }
}
