use crate::{GameState, engine::piece::Piece, input::MoveMade};
use bevy::{
    color::palettes::{
        css::GREEN,
        tailwind::{STONE_200, STONE_700},
    },
    gizmos::gizmos::Gizmos,
    platform::collections::HashMap,
    prelude::*,
};
use std::f32::consts::FRAC_PI_2;

/// A plugin for displaying pieces throughout the game.
pub struct PiecePlugin;

/// Hashmap for quickly looking up the entity id of a piece via its board index.
#[derive(Resource, Default)]
pub struct Pieces(HashMap<u8, Entity>);

/// Marker for pieces that are moving to a new destination square.
#[derive(Component, Debug)]
struct Moving {
    /// The curve the piece should move along.
    curve: CubicCurve<Vec3>,
    /// Duration of the move.
    timer: Timer,
}

/// Handles for piece related meshes and materials.
#[derive(Resource)]
pub struct PieceHandles {
    king_mesh: Handle<Mesh>,
    queen_mesh: Handle<Mesh>,
    pawn_mesh: Handle<Mesh>,
    bishop_mesh: Handle<Mesh>,
    knight_mesh: Handle<Mesh>,
    rook_mesh: Handle<Mesh>,
    white_material: Handle<StandardMaterial>,
    black_material: Handle<StandardMaterial>,
}

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pieces>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    curvetest,
                    move_pieces,
                    apply_moves_to_game_state.run_if(resource_exists::<PieceHandles>),
                ),
            );
    }
}

fn curvetest(mut gizmos: Gizmos) {
    // Try to draw a curve with gizmos.
    let points = vec![
        vec3(0., 0., 0.),
        vec3(1.0, 2., 1.0),
        vec3(6.0, 2., 6.0),
        vec3(7.0, 0.0, 7.0),
    ];
    let spline = CubicCardinalSpline::new_catmull_rom(points);
    let curve = spline.to_curve().expect("to be able to form curve");
    let resolution = 10 * curve.segments().len();
    for p in curve.iter_positions(resolution) {
        gizmos.circle(Isometry3d::from_translation(p), 0.02, GREEN);
    }
    gizmos.linestrip(curve.iter_positions(resolution), Color::srgb(1.0, 1.0, 1.0));
}

fn setup(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut pieces: ResMut<Pieces>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let handles = PieceHandles {
        king_mesh: asset_server.load("models/pieces.glb#Mesh0/Primitive0"),
        queen_mesh: asset_server.load("models/pieces.glb#Mesh2/Primitive0"),
        pawn_mesh: asset_server.load("models/pieces.glb#Mesh1/Primitive0"),
        bishop_mesh: asset_server.load("models/pieces.glb#Mesh3/Primitive0"),
        knight_mesh: asset_server.load("models/pieces.glb#Mesh4/Primitive0"),
        rook_mesh: asset_server.load("models/pieces.glb#Mesh5/Primitive0"),
        white_material: materials.add(StandardMaterial {
            metallic: 0.6,
            perceptual_roughness: 0.5,
            base_color: STONE_200.into(),
            ..default()
        }),
        black_material: materials.add(StandardMaterial {
            metallic: 0.6,
            perceptual_roughness: 0.5,
            base_color: STONE_700.into(),
            ..default()
        }),
    };
    for r in 0..game_state.width {
        for c in 0..game_state.height {
            let square = (r * 8 + c) as u8;
            if let Some(piece) = game_state.board.at(square) {
                spawn_piece(
                    &mut commands,
                    &game_state,
                    &mut pieces,
                    &handles,
                    piece,
                    square,
                );
            }
        }
    }
    commands.insert_resource(handles);
}

/// Spawns a piece on a square.
/// This is not a system!
fn spawn_piece(
    commands: &mut Commands,
    game_state: &GameState,
    pieces: &mut Pieces,
    handles: &PieceHandles,
    piece: Piece,
    square: u8,
) {
    let (r, c) = (
        square / game_state.width as u8,
        square % game_state.width as u8,
    );
    let mesh = match piece {
        Piece::PawnWhite | Piece::PawnBlack => handles.pawn_mesh.clone(),
        Piece::KnightWhite | Piece::KnightBlack => handles.knight_mesh.clone(),
        Piece::BishopWhite | Piece::BishopBlack => handles.bishop_mesh.clone(),
        Piece::RookWhite | Piece::RookBlack => handles.rook_mesh.clone(),
        Piece::QueenWhite | Piece::QueenBlack => handles.queen_mesh.clone(),
        Piece::KingWhite | Piece::KingBlack => handles.king_mesh.clone(),
    };
    let material = if piece.is_white() {
        handles.white_material.clone()
    } else {
        handles.black_material.clone()
    };
    let rotation = if piece.is_white() {
        FRAC_PI_2
    } else {
        -FRAC_PI_2
    };
    let transform =
        Transform::from_xyz(r as f32, 0.0, c as f32).with_rotation(Quat::from_rotation_y(rotation));
    let entity = commands
        .spawn((Mesh3d(mesh), MeshMaterial3d(material), transform))
        .id();
    pieces.0.insert(square as u8, entity);
}

fn apply_moves_to_game_state(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut moves: EventReader<MoveMade>,
    mut pieces: ResMut<Pieces>,
    mut transforms: Query<&mut Transform>,
    handles: Res<PieceHandles>,
) {
    for m in moves.read() {
        let move_info = game_state.board.apply(m.0);
        info!("{:?}", move_info);

        for removed in move_info.removed_pieces {
            // Remove the entity.
            let mut commands = pieces
                .0
                .remove(&removed)
                .and_then(|entity| commands.get_entity(entity).ok())
                .expect("to be able to get entity commands");

            commands.try_despawn();
        }

        for (square, piece) in move_info.added_pieces {
            // Spawn the piece
            spawn_piece(
                &mut commands,
                &game_state,
                &mut pieces,
                &handles,
                piece,
                square,
            );
        }

        for (from, to) in move_info.moved_pieces {
            let moving_piece = pieces.0.remove(&from).expect("to be moving a piece");
            let current = vec3(
                (from as usize / game_state.width) as f32,
                0.0,
                (from as usize % game_state.width) as f32,
            );
            let target = vec3(
                (to as usize / game_state.width) as f32,
                0.0,
                (to as usize % game_state.width) as f32,
            );
            let distance = target - current;
            let top1 = (current + distance * 0.1).with_y((0.25 * distance.length()).min(2.0));
            let top2 = (target - distance * 0.1).with_y((0.25 * distance.length()).min(2.0));
            let points = vec![current, top1, top2, target];
            let spline = CubicCardinalSpline::new_catmull_rom(points);
            let curve = spline.to_curve().expect("to be able to form curve");
            let timer = Timer::from_seconds(0.25 * distance.length(), TimerMode::Once);
            commands
                .entity(moving_piece)
                .insert(Moving { curve, timer });

            // Update pieces hashmap.
            pieces.0.remove(&to);
            pieces.0.insert(to, moving_piece);
        }
    }
}

fn move_pieces(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(&mut Transform, &mut Moving, Entity), With<Moving>>,
) {
    for (mut transform, mut moving, entity) in query {
        moving.timer.tick(time.delta());
        let position = moving.curve.position(moving.timer.fraction() * 3.0);
        transform.translation = position;
        if moving.timer.finished() {
            // Remove `Moving` component, since the animation ended.
            commands.entity(entity).remove::<Moving>();
        }
    }
}
