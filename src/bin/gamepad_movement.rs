use bevy::prelude::*;
// Not yet supported in 0.16
// use bevy_editor_pls::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Not yet supported in 0.16
        // .add_plugins(EditorPlugin::default())
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (player_movement, camera_movement))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(pub f32);

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 1., 1.),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands
        .spawn((
            // Mesh3d(meshes.add(Rect::new(0., 0., 10., 10.))),
            Mesh3d(meshes.add(Sphere::new(2.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_xyz(0., 0., 3.),
        ))
        .insert(Speed(15.))
        .insert(Player);

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., -60., 60.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add a light source for better 3d visibility.
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-15., 30., 50.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn player_movement(
    gamepads: Query<(Entity, &Gamepad)>,
    mut spheres: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (entity, gamepad) in &gamepads {
        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_x.abs() > 0.01 || left_stick_y.abs() > 0.01 {
            info!(
                "Current position of {} left stick is {}, {}",
                entity, left_stick_x, left_stick_y
            );

            for (mut transform, speed) in &mut spheres {
                let current_x = transform.translation.x;
                let current_y = transform.translation.y;
                transform.translation = Vec3::new(
                    current_x + left_stick_x * speed.0 * dt,
                    current_y + left_stick_y * speed.0 * dt,
                    transform.translation.z,
                );
            }
        }
    }
}

fn camera_movement(
    gamepads: Query<(Entity, &Gamepad)>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    for (_entity, gamepad) in &gamepads {
        for mut transform in &mut cameras {
            let current_x = transform.translation.x;
            let current_y = transform.translation.y;
            let current_z = transform.translation.z;

            if gamepad.just_pressed(GamepadButton::DPadLeft) {
                info!("DPadLeft pressed");
                let new_translation = Vec3::new(current_x + 5., current_y, current_z);
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
            if gamepad.just_pressed(GamepadButton::DPadRight) {
                info!("DPadRight pressed");
                let new_translation = Vec3::new(current_x - 5., current_y, current_z);
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
            if gamepad.just_pressed(GamepadButton::DPadUp) {
                info!("DPadUp pressed");
                let new_translation = Vec3::new(current_x, current_y + 5., current_z);
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
            if gamepad.just_pressed(GamepadButton::DPadDown) {
                info!("DPadDown pressed");
                let new_translation = Vec3::new(current_x, current_y - 5., current_z);
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
            if gamepad.just_pressed(GamepadButton::RightTrigger) {
                info!("Right trigger pressed");
                let new_translation = Vec3::new(current_x, current_y, current_z + 5.);
                // Adjust the translation to maintain the correct orientation toward the orbit target.
                // In our example it's a static target, but this could easily be customized.
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
            if gamepad.just_pressed(GamepadButton::LeftTrigger) {
                info!("Left trigger pressed");
                let new_translation = Vec3::new(current_x, current_y, current_z - 5.);
                // Adjust the translation to maintain the correct orientation toward the orbit target.
                // In our example it's a static target, but this could easily be customized.
                *transform = transform
                    .with_translation(new_translation)
                    .looking_at(Vec3::ZERO, Vec3::Y);
            }
        }
    }
}
