use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
};

fn main() {
    App::build()
        .add_default_plugins()
        .add_system_init(mouse_input_system)
        .run();
}

/// prints out mouse events as they come in
pub fn mouse_input_system(resources: &mut Resources) -> Box<dyn Schedulable> {
    let mut mouse_button_input_event_reader = resources.get_event_reader::<MouseButtonInput>();
    let mut mouse_motion_event_reader = resources.get_event_reader::<MouseMotion>();
    SystemBuilder::new("mouse_input")
        .read_resource::<Events<MouseButtonInput>>()
        .read_resource::<Events<MouseMotion>>()
        .build(
            move |_command_buffer,
                  _world,
                  (mouse_button_input_events, mouse_motion_events),
                  _queries| {
                for event in mouse_button_input_event_reader.iter(&mouse_button_input_events) {
                    println!("{:?}", event);
                }

                for event in mouse_motion_event_reader.iter(&mouse_motion_events) {
                    println!("{:?}", event);
                }
            },
        )
}