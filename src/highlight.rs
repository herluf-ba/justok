use std::f32::consts::FRAC_PI_2;

use crate::{
    GameState,
    input::{DeselectSquare, SelectSquare},
};
use bevy::{pbr::NotShadowCaster, platform::collections::HashMap, prelude::*};

/// A plugin that highlights certain squares on the board.
pub struct HighlightPlugin;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SquareHighlights>()
            .add_systems(Startup, setup)
            .add_systems(Update, (on_select, on_deselect));
    }
}

/// Hashmap for quickly looking up the material handle
/// of the highlight for a square via its index.
#[derive(Resource, Default)]
pub struct SquareHighlights(HashMap<u8, Handle<StandardMaterial>>);

fn on_select(
    game_state: Res<GameState>,
    square_highlights: Res<SquareHighlights>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<SelectSquare>,
) {
    for e in events.read() {
        let material = square_highlights
            .0
            .get(&e.0)
            .and_then(|h| materials.get_mut(h))
            .expect("square highlight to exist");
        material.base_color.set_alpha(1.0);

        // Also highlight squares the piece can move to.
        let valid_moves = game_state.board.generate_square_moves(e.0);
        let target_squares = valid_moves.iter().map(|m| m.to);
        for s in target_squares {
            let material = square_highlights
                .0
                .get(&s)
                .and_then(|h| materials.get_mut(h))
                .expect("square highlight to exist");
            material.base_color.set_alpha(0.65);
        }
    }
}

fn on_deselect(
    square_highlights: Res<SquareHighlights>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut events: EventReader<DeselectSquare>,
) {
    for _ in events.read() {
        for m_h in square_highlights.0.values() {
            let material = materials
                .get_mut(m_h)
                .expect("square highlight material to exist");
            material.base_color.set_alpha(0.0);
        }
    }
}

fn setup(
    game_state: Res<GameState>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut square_highlights: ResMut<SquareHighlights>,
    asset_server: Res<AssetServer>,
) {
    let block_highlight_plane = meshes.add(Rectangle::new(1.0, 1.0));
    let tex_square = asset_server.load("textures/square.png");

    for r in 0..game_state.width {
        for c in 0..game_state.height {
            // Spawn plane for higlighting.
            let mat_highlight = materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                base_color_texture: Some(tex_square.clone()),
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            commands.spawn((
                Mesh3d(block_highlight_plane.clone()),
                MeshMaterial3d(mat_highlight.clone()),
                Transform::from_xyz(r as f32, 0.01, c as f32)
                    .with_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
                NotShadowCaster,
            ));
            // Insert the material handle into hashmap so we can look it up later.
            square_highlights.0.insert((r * 8 + c) as u8, mat_highlight);
        }
    }
}
