//! Demonstrates how to change the ordering of observers

use bevy::log::LogPlugin;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(LogPlugin::default());

    app.add_observer(add_examine_text_1);
    app.add_observer(add_examine_text_2);
    app.add_observer(add_examine_text_3);
    app.add_observer(add_examine_text_4);
    app.add_observer(add_examine_text_5);

    app.add_systems(Startup, do_examine);

    app.run();
}

fn my_system() {

}

#[derive(Event)]
struct ExamineEvent(String);

fn do_examine(world: &mut World) {
    let mut examine = ExamineEvent(String::new());

    world.trigger_ref(&mut examine);

    info!("Examine Text: {}", examine.0);

    // assert_eq!(examine.0.as_str(), "123");
}

fn add_examine_text_1(mut examine: On<ExamineEvent>) {
    examine.event_mut().0.push_str("1");
}

fn add_examine_text_2(mut examine: On<ExamineEvent>) {
    examine.event_mut().0.push_str("2");
}

fn add_examine_text_3(mut examine: On<ExamineEvent>) {
    examine.event_mut().0.push_str("3");
}

fn add_examine_text_4(mut examine: On<ExamineEvent>) {
    examine.event_mut().0.push_str("4");
}

fn add_examine_text_5(mut examine: On<ExamineEvent>) {
    examine.event_mut().0.push_str("5");
}
