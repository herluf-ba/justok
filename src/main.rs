mod camera;
mod engine;
mod highlight;
mod input;
mod pieces;

use bevy::{color::palettes::tailwind::*, pbr::DirectionalLightShadowMap, prelude::*};
use camera::CameraControllerPlugin;
use engine::board::Board;
use highlight::HighlightPlugin;
use input::{InputPlugin, MoveMade};
use pieces::PiecePlugin;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::FRAC_PI_3;

#[derive(Resource)]
pub struct GameState {
    pub board: Board,
    pub width: usize,
    pub height: usize,
}

#[derive(Resource)]
pub struct Rand(pub ChaCha8Rng);

impl Default for GameState {
    fn default() -> Self {
        // "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
        return Self {
            board: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            width: 8,
            height: 8,
        };
    }
}

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins((
            DefaultPlugins,
            CameraControllerPlugin,
            InputPlugin,
            HighlightPlugin,
            PiecePlugin,
        ))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Rand(ChaCha8Rng::seed_from_u64(19878367467713)))
        .add_systems(Startup, (setup_environment, setup_board))
        .add_systems(Update, play_as_black)
        .run();
}

fn play_as_black(
    game_state: Res<GameState>,
    mut events: EventWriter<MoveMade>,
    mut rng: ResMut<Rand>,
) {
    if game_state.board.white_to_move {
        // Bot is black so do nothing.
        return;
    }

    let moves = game_state.board.generate_moves();
    if moves.len() > 0 {
        let idx = rng.0.random_range(0..moves.len());
        events.write(MoveMade(moves[idx]));
    }
}

fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Light"),
        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 0.85, -FRAC_PI_3)),
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 6000.0,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Plane"),
        Transform::from_xyz(0.0, -0.5, 0.0),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: GREEN_700.into(),
            metallic: 0.1,
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));
}

fn setup_board(
    game_state: Res<GameState>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let block_mesh: Handle<Mesh> = asset_server.load("models/board.gltf#Mesh0/Primitive0");
    let block_light_material = materials.add(StandardMaterial {
        metallic: 0.1,
        perceptual_roughness: 0.9,
        base_color: SLATE_400.into(),
        ..default()
    });

    let block_dark_material = materials.add(StandardMaterial {
        metallic: 0.1,
        perceptual_roughness: 0.9,
        base_color: SLATE_600.into(),
        ..default()
    });

    for r in 0..game_state.width {
        for c in 0..game_state.height {
            let is_white = (r % 2 == 0) == (c % 2 == 0);
            let material = if is_white {
                block_light_material.clone()
            } else {
                block_dark_material.clone()
            };

            commands.spawn((
                Mesh3d(block_mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(r as f32, -0.25, c as f32),
            ));
        }
    }
}
