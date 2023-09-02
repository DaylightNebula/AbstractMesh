/* use bevy::prelude::*;

// create a struct for our plugin
pub struct HelloPlugin;

// create plugin that extends Plugin and adds all systems
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)// Adds a function that runs on startup
            .add_system(greet_people);              // Adds a function that is run in parallel at a fixed rate
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)    // Adds plugins
        .add_plugin(HelloPlugin)        // Add a single plugin
        .run();                                         // run everything
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0)
        }
    }
} */