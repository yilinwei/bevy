use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const BIRDS_PER_SECOND: u32 = 1000;
const GRAVITY: f32 = -9.8 * 100.0;
const MAX_VELOCITY: f32 = 750.;
const BIRD_SCALE: f32 = 0.15;
const HALF_BIRD_SIZE: f32 = 256. * BIRD_SCALE * 0.5;
struct BevyCounter {
    pub count: u128,
}

struct Bird {
    velocity: Vec3,
}

struct BirdMaterial(Handle<ColorMaterial>);

impl FromResources for BirdMaterial {
    fn from_resources(resources: &Resources) -> Self {
        let mut color_materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let asset_server = resources.get_mut::<AssetServer>().unwrap();
        BirdMaterial(color_materials.add(asset_server.load("branding/icon.png").into()))
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "BevyMark".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_resource(BevyCounter { count: 0 })
        .init_resource::<BirdMaterial>()
        .add_startup_system(setup.system())
        .add_system(mouse_handler.system())
        .add_system(movement_system.system())
        .add_system(collision_system.system())
        .add_system(counter_system.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(UiCameraBundle::default())
        .spawn(TextBundle {
            text: Text {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                value: "Bird Count:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    font_size: 40.0,
                    ..Default::default()
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });
}

fn mouse_handler(
    commands: &mut Commands,
    time: Res<Time>,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Res<WindowDescriptor>,
    bird_material: Res<BirdMaterial>,
    mut counter: ResMut<BevyCounter>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        let spawn_count = (BIRDS_PER_SECOND as f32 * time.delta_seconds) as u128;
        let bird_x = (window.width as i32 / -2) as f32 + HALF_BIRD_SIZE;
        let bird_y = (window.height / 2) as f32 - HALF_BIRD_SIZE;

        for count in 0..spawn_count {
            let bird_position = Vec3::new(bird_x, bird_y, (counter.count + count) as f32 * 0.00001);
            let mut transform = Transform::from_translation(bird_position);
            transform.scale = Vec3::new(BIRD_SCALE, BIRD_SCALE, BIRD_SCALE);

            commands
                .spawn(SpriteBundle {
                    material: bird_material.0.clone(),
                    transform,
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Bird {
                    velocity: Vec3::new(
                        rand::random::<f32>() * MAX_VELOCITY - (MAX_VELOCITY * 0.5),
                        0.,
                        0.,
                    ),
                });
        }

        counter.count += spawn_count;
    }
}

fn movement_system(time: Res<Time>, mut bird_query: Query<(&mut Bird, &mut Transform)>) {
    for (mut bird, mut transform) in bird_query.iter_mut() {
        *transform.translation.x_mut() += bird.velocity.x() * time.delta_seconds;
        *transform.translation.y_mut() += bird.velocity.y() * time.delta_seconds;
        *bird.velocity.y_mut() += GRAVITY * time.delta_seconds;
    }
}

fn collision_system(window: Res<WindowDescriptor>, mut bird_query: Query<(&mut Bird, &Transform)>) {
    let half_width = window.width as f32 * 0.5;
    let half_height = window.height as f32 * 0.5;

    for (mut bird, transform) in bird_query.iter_mut() {
        let x_vel = bird.velocity.x();
        let y_vel = bird.velocity.y();
        let x_pos = transform.translation.x();
        let y_pos = transform.translation.y();

        if (x_vel > 0. && x_pos + HALF_BIRD_SIZE > half_width)
            || (x_vel <= 0. && x_pos - HALF_BIRD_SIZE < -(half_width))
        {
            bird.velocity.set_x(-x_vel);
        }
        if y_vel < 0. && y_pos - HALF_BIRD_SIZE < -half_height {
            bird.velocity.set_y(-y_vel);
        }
    }
}

fn counter_system(
    diagnostics: Res<Diagnostics>,
    counter: Res<BevyCounter>,
    mut query: Query<&mut Text>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            for mut text in query.iter_mut() {
                text.value = format!("Bird Count: {}\nAverage FPS: {:.2}", counter.count, average);
            }
        }
    };
}