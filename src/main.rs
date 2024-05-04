use bevy::prelude::*;
use bevy::time::Timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: "AAA Fancy_splash_screen".into(),
                    resolution: (1920.0,1200.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .build(),
        )  
        .add_systems(Startup, setup)
        .add_systems(Update, fade_system)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);

    // Charger toutes les images
    let image_paths = vec![
        "img\\bevy_logo.png",
        "img\\rust_logo.png",
        "img\\wpixo_logo.png",
    ];
    let images: Vec<Handle<Image>> = image_paths
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();

    // Créer une entité avec la première image et le composant Fade
    commands.spawn(SpriteBundle {
        texture: images[0].clone(),
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    })
    .insert(Fade {
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        fading_in: true,
        image_index: 0,
        images,
    });
}

#[derive(Component)]
struct Fade {
    timer: Timer,
    fading_in: bool,
    image_index: usize,
    images: Vec<Handle<Image>>,
}

fn fade_system(
    time: Res<Time>, 
    // asset_server: Res<AssetServer>,
    mut query: Query<(&mut Fade, &mut Handle<Image>, &mut Sprite)>,
) {
    for (mut fade, mut texture, mut sprite) in query.iter_mut() {
        fade.timer.tick(time.delta());
        let elapsed = fade.timer.elapsed_secs();
        let duration = fade.timer.duration().as_secs_f32();
        let percent = elapsed / duration;

        if fade.fading_in {
            sprite.color.set_a(percent.clamp(0.0, 1.0));
        } else {
            sprite.color.set_a((1.0 - percent).clamp(0.0, 1.0));
        }

        // Check if a full cycle has completed
        if fade.timer.just_finished() {
            if fade.fading_in {
                // Just finished fading in
                fade.fading_in = false;
            } else {
                // Just finished fading out, change image
                fade.image_index = (fade.image_index + 1) % fade.images.len();
                *texture = fade.images[fade.image_index].clone();
                fade.fading_in = true;
            }
        }
    }
}
