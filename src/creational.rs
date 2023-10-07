use druid::widget::{Flex, Label, RadioGroup};
use druid::{Widget, WidgetExt};

pub fn display_patterns() {
    let patterns = vec![
        ("Singleton", "singleton"),
        ("Builder", "builder"),
        ("Prototype", "prototype"),
        ("Factory Method", "factory_method"),
        ("Abstract Factory", "abstract_factory"),
    ];

    let radio_group = RadioGroup::new(patterns).on_changed(|_ctx, data, _env| {
        match *data {
            "singleton" => println!("Singleton pattern selected!"),
            "builder" => println!("Builder pattern selected!"),
            // ... (similar lines for other patterns)
            _ => {}
        }
    });

    let label = Label::new("Select a creational design pattern:");
    Flex::column()
        .with_child(label)
        .with_spacer(8.0)
        .with_child(radio_group);
}