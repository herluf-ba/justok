use crate::{
    GameState,
    engine::{r#move::Move, piece::Piece},
};
use bevy::prelude::*;

/// A plugin that detects player input and spawns matching events for other systems to react to.
pub struct InputPlugin;

/// Index of currently selected square. This is updated _on the frame after the click happened_.
#[derive(Resource, Default, Debug)]
pub struct SelectedSquare(pub Option<u8>);

/// This event will be sent when a square has been selected.
#[derive(Event, Debug)]
pub struct SelectSquare(pub u8);

/// This event will be sent when a square has been deselected.
#[derive(Event, Debug)]
pub struct DeselectSquare(pub u8);

/// This event will be sent when a move is made.
#[derive(Event, Debug)]
pub struct MoveMade(pub Move);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .add_event::<SelectSquare>()
            .add_event::<DeselectSquare>()
            .add_event::<MoveMade>()
            .add_systems(Update, log_events)
            .add_systems(Update, (handle_click, update_selected_square));
    }
}

/// Determines which square on the board, if any, was clicked
/// by casting a ray along the camera's forward vector and finding its
/// intersection with the ground plane.
fn handle_click(
    game_state: Res<GameState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    selected_square: Res<SelectedSquare>,
    mut select_events: EventWriter<SelectSquare>,
    mut deselect_events: EventWriter<DeselectSquare>,
    mut move_events: EventWriter<MoveMade>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Ok(windows) = windows.single() else {
            return;
        };
        let (camera, camera_transform) = *camera_query;
        let Some(cursor_position) = windows.cursor_position() else {
            return;
        };

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };

        // Calculate if and where the ray is hitting the ground plane.
        let Some(distance) = ray.intersect_plane(Vec3::splat(0.0), InfinitePlane3d::new(Vec3::Y))
        else {
            return;
        };

        // Calculate the square that was clicked.
        let point = ray.get_point(distance);
        let r = (point.x + 0.5).floor();
        let c = (point.z + 0.5).floor();
        let is_in_bounds =
            r >= 0.0 && r < game_state.width as f32 && c >= 0.0 && c < game_state.height as f32;

        if !is_in_bounds {
            // Click was not on the board. Deselect the currently selected square if any.
            if let Some(s) = selected_square.0 {
                deselect_events.write(DeselectSquare(s));
            }
            return;
        }

        let target = r as u8 * 8 + c as u8;
        let square_has_friendly_piece = game_state
            .board
            .at(target)
            .is_some_and(|p| p.is_white() == game_state.board.white_to_move);

        if let Some(selected) = selected_square.0 {
            if selected == target {
                return; // Re-clicking is neutral.
            }

            // Handle selecting another piece.
            if square_has_friendly_piece {
                select_events.write(SelectSquare(target));
                deselect_events.write(DeselectSquare(selected));
                return;
            }

            // Handle making a move.
            let is_pawn_move = game_state
                .board
                .at(selected)
                .is_some_and(|p| p == Piece::PawnWhite);
            let is_moving_to_back_rank =
                target > ((game_state.width * game_state.height - 1) - game_state.width) as u8;

            let promote_to = if is_pawn_move && is_moving_to_back_rank {
                Some(Piece::QueenWhite) // Auto promote to queens for now.
            } else {
                None
            };
            let m = Move {
                from: selected,
                to: target,
                promote_to,
            };
            let moves = game_state.board.generate_moves();
            let is_valid = moves.iter().find(|&vm| *vm == m).is_some();
            if is_valid {
                move_events.write(MoveMade(m));
            }
            deselect_events.write(DeselectSquare(selected));
            return;
        }

        // If there wasn't a selected square,
        // select the clicked one if it has a friendly piece on it.
        if square_has_friendly_piece {
            select_events.write(SelectSquare(target));
        }
    }
}

fn update_selected_square(
    mut selected_square: ResMut<SelectedSquare>,
    mut selects: EventReader<SelectSquare>,
    mut deselects: EventReader<DeselectSquare>,
) {
    for e in selects.read() {
        selected_square.0 = Some(e.0);
    }
    for e in deselects.read() {
        if selected_square.0.is_some_and(|s| s == e.0) {
            selected_square.0 = None;
            // We don't want to break here, because we still want to read all the accumulated deselects.
        }
    }
}

fn log_events(
    mut selects: EventReader<SelectSquare>,
    mut deselects: EventReader<DeselectSquare>,
    mut moves: EventReader<MoveMade>,
) {
    for e in selects.read() {
        info!("{:?}", e);
    }
    for e in deselects.read() {
        info!("{:?}", e);
    }

    for e in moves.read() {
        info!("{:?}", e);
    }
}
