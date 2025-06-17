use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

/// An orbit camera controller plugin.
pub struct CameraControllerPlugin;

// TODO: Use GameState resource for these
const BOARD_WIDTH: usize = 8;
const BOARD_HEIGHT: usize = 8;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State {
            target: Vec3::new(
                (BOARD_WIDTH / 2) as f32 - 0.5,
                0.0,
                (BOARD_HEIGHT / 2) as f32 - 0.5,
            ),
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, orbit);
    }
}

#[derive(Debug, Resource)]
struct State {
    /// The look target of the camera.
    pub target: Vec3,
    /// The distance maintained to `target` along the cameras forward vector.
    pub orbit_distance: f32,
    /// yaw speed in radians.
    pub yaw_speed: f32,
    /// current yaw velocity.
    pub yaw_velocity: f32,
    /// The rate at which yaw velocity decays when no longer dragging.
    /// Value of 0.5 means the velocity halves each frame.
    pub friction: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            orbit_distance: 13.5,
            yaw_speed: 0.002,
            yaw_velocity: 0.0,
            friction: 0.25,
        }
    }
}

fn setup(mut commands: Commands, state: Res<State>, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_euler(
        EulerRot::YXZ,
        -FRAC_PI_2,
        -FRAC_PI_3,
        0.0,
    ));
    transform.translation = state.target - transform.forward() * state.orbit_distance;

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        transform,
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 4500.0,
            ..default()
        },
    ));
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    mut state: ResMut<State>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
) {
    if mouse_buttons.pressed(MouseButton::Left) && mouse_motion.delta.x.abs() > 0.0 {
        // If mouse is pressed set the yaw velocity to its max value.
        state.yaw_velocity = mouse_motion.delta.x * state.yaw_speed;
    } else {
        // If not dragging decay the velocity based on the friciton setting.
        state.yaw_velocity *= 1.0 - state.friction;
        if state.yaw_velocity.abs() < 1e-6 {
            state.yaw_velocity = 0.0;
        }
    }
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw + state.yaw_velocity, pitch, roll);
    camera.translation = state.target - camera.forward() * state.orbit_distance;
}
