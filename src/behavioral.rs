use druid::widget::{Flex, Label, RadioGroup};
use druid::{Widget, WidgetExt};

pub fn display_patterns() {
    let patterns = vec![
        ("Chain of Responsibility", "chain_of_responsibility"),
        ("Command", "command"),
        ("Interpreter", "interpreter"),
        ("Iterator", "iterator"),
        ("Mediator", "mediator"),
        ("Memento", "memento"),
        ("Observer", "observer"),
        ("State", "state"),
        ("Strategy", "strategy"),
        ("Template Method", "template_method"),
        ("Visitor", "visitor"),
    ];

    let radio_group = RadioGroup::new(patterns).on_changed(|_ctx, data, _env| {
        match *data {
            "chain_of_responsibility" => println!("Chain of Responsibility pattern selected!"),
            "command" => println!("Command pattern selected!"),
            // ... (similar lines for other patterns)
            _ => {}
        }
    });

    let label = Label::new("Select a behavioral design pattern:");
    Flex::column()
        .with_child(label)
        .with_spacer(8.0)
        .with_child(radio_group);
}