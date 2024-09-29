use bevy::{
    asset::AssetServer,
    math::Vec2,
    prelude::{ Bundle, Component, Res },
    sprite::{ Sprite, SpriteBundle },
    transform::components::Transform,
};

// #[derive(Component)]
// struct Map {
//     map: Vec<Vec<Tile>>,
// }

// impl Map {
//     fn generate(self) -> Vec<Vec<Tiles>> {
//         for row in 0..199 {
//             for col in 0..199 {
//                 self.map[row][col] = Tile::new();
//             }
//         }
//         return self.map;
//     }
// }

// #[derive(Component)]
// struct Tile {
//     stack: Vec<Item>,
//     coord: Coordinates,
// }

// impl Tile {
//     fn draw(&self) {
//         for item in self.stack {
//         }
//     }
// }

#[derive(Component)]
pub struct Item {
    name: String,
}

impl Item {
    pub fn new(name: String) -> Self {
        Item { name }
    }
}

#[derive(Bundle)]
pub struct ItemBundle {
    item: Item,
    sprite: SpriteBundle,
}

impl ItemBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        item: Item,
        texture_path: String,
        coordinates: Coordinates
    ) -> Self {
        let Coordinates(x, y, z) = coordinates;
        let transform = Transform::from_xyz(x, y, z);
        let texture = asset_server.load(texture_path);
        ItemBundle {
            item,
            sprite: SpriteBundle {
                texture,
                transform,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub struct Coordinates(pub f32, pub f32, pub f32);
