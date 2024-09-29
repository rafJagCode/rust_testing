use bevy::{ prelude::Bundle, time::Timer };
use crate::components::marker_components::{ Player, TileSize };

// #[derive(Component)]
// struct Player {
//     delta_x: f32,
//     delta_y: f32,
//     move_coldown: Timer,
// }

// impl Player {
//     fn new() -> Self {
//         Player {
//             delta_x: 0.0,
//             delta_y: 0.0,
//             move_coldown: Timer::new(Duration::from_millis(100), TimerMode::Once),
//         }
//     }
// }

// impl ItemBundle {
//     pub fn new(
//         asset_server: Res<AssetServer>,
//         item: Item,
//         texture_path: String,
//         coordinates: Coordinates
//     ) -> Self {
//         let Coordinates(x, y, z) = coordinates;
//         let transform = Transform::from_xyz(x, y, z);
//         let texture = asset_server.load(texture_path);
//         ItemBundle {
//             item,
//             sprite: SpriteBundle {
//                 texture,
//                 transform,
//                 sprite: Sprite {
//                     custom_size: Some(Vec2::new(100.0, 100.0)),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             },
//         }
//     }
// }

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: Player,
    tile_size_marker: TileSize,
    move_controller: MoveController,
    sprite: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player_marker: Player,
            tile_size_marker: TileSize,
            speed: Speed,
            sprite: PlayerSprite,
        }
    }
}

struct MoveController {
    speed: f32,
    coldown: Timer,
}

struct PlayerSprite {}
