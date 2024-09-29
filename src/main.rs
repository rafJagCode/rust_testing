use std::time::Duration;
use bevy::app::App;
use bevy::sprite::{ MaterialMesh2dBundle, Mesh2dHandle };
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::{ PresentMode, Window, WindowMode, WindowPlugin, WindowResized, WindowTheme };
mod components;
use components::{ Player, PlayerBundle };

#[derive(Component)]
struct GameSettings {
    window_width: f32,
    window_height: f32,
    tiles_per_width: f32,
    tile_size: f32,
}

impl GameSettings {
    fn new() -> Self {
        GameSettings {
            window_width: 800.0,
            window_height: 800.0,
            tiles_per_width: 100.0,
            tile_size: 8.0,
        }
    }
}

#[derive(Component)]
struct Player {
    delta_x: f32,
    delta_y: f32,
    move_coldown: Timer,
}

impl Player {
    fn new() -> Self {
        Player {
            delta_x: 0.0,
            delta_y: 0.0,
            move_coldown: Timer::new(Duration::from_millis(100), TimerMode::Once),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Game".to_string(),
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(Update, handle_window_resize)
        // .add_systems(Update, draw_corners)
        .add_systems(Update, handle_key_events)
        .add_systems(Update, move_player)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let game_settings = GameSettings::new();
    let tile_size = game_settings.tile_size;
    let player = Player::new();
    let field = ItemBundle::new(
        asset_server,
        Item::new("tile".to_string()),
        "tiles/Tile_67.png".to_string(),
        Coordinates(20.0, 20.0, 0.0)
    );

    commands.spawn(field);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(game_settings);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(tile_size, tile_size))),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(Color::hsl(360.0, 0.95, 0.7)),
            ..default()
        },
        player,
    ));
}

fn handle_key_events(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut game_settings_query: Query<&GameSettings>,
    time: Res<Time>
) {
    let mut player = player_query.single_mut();
    if player.move_coldown.tick(time.delta()).finished() {
        let game_settings = game_settings_query.single_mut();
        let tile_size = game_settings.tile_size;

        for ev in keyboard_input.get_pressed() {
            match ev {
                KeyCode::KeyW => {
                    player.delta_y += tile_size;
                }
                KeyCode::KeyA => {
                    player.delta_x -= tile_size;
                }
                KeyCode::KeyS => {
                    player.delta_y -= tile_size;
                }
                KeyCode::KeyD => {
                    player.delta_x += tile_size;
                }
                _ => {}
            }
        }
        player.move_coldown.reset();
    }
}

fn move_player(mut query: Query<(&mut Player, &mut Transform)>) {
    let (mut player, mut transform) = query.single_mut();
    transform.translation.x += player.delta_x;
    transform.translation.y += player.delta_y;
    player.delta_x = 0.0;
    player.delta_y = 0.0;
}

fn handle_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut game_settings_query: Query<&mut GameSettings>,
    mut player_query: Query<&mut Mesh2dHandle, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let mut game_settings = game_settings_query.single_mut();
    let mut mesh = player_query.single_mut();
    for e in resize_reader.read() {
        game_settings.window_width = e.width;
        game_settings.window_height = e.height;
        game_settings.tile_size = e.width / game_settings.tiles_per_width;
        *mesh = Mesh2dHandle(
            meshes.add(Rectangle::new(game_settings.tile_size, game_settings.tile_size))
        );
    }
}
