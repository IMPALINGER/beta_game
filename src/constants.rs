pub const TILE_SIZE: f32 = 16.0;
pub const SPRITE_SCALE: f32 = 4.0;
pub const Z_BASE_FLOOR: f32 = 100.0; // Base z-coordinate for 2D layers.
pub const Z_BASE_OBJECTS: f32 = 200.0; // Ground object sprites.
pub const SCREEN_SIZE: (f32, f32) = (1280.0, 720.0);
pub const CAMERA_SCALE: f32 = 1.5;
pub const CAMERA_SCALE_BOUNDS: (f32, f32) = (1., 20.);
pub const CAMERA_ZOOM_SPEED: f32 = 3.;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 1000;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const SPAWN_RATE_PER_SECOND: usize = 10;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;

pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const TILE_H: u32 = 16;
pub const TILE_W: u32 = 16;
pub const SPRITE_SHEET_H: u32 = 4;
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const SPRITE_SHEET_W: u32 = 4;

pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;
pub const BG_COLOR: (u8, u8, u8) = (25, 20, 43);

pub const PLAYER_SPEED: f32 = 2.0;
