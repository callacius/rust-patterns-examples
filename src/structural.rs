use druid::widget::{Flex, Label, RadioGroup};
use druid::{Widget, WidgetExt};

pub fn display_patterns() {
    let patterns = vec![
        ("Adapter", "adapter"),
        ("Bridge", "bridge"),
        ("Composite", "composite"),
        ("Decorator", "decorator"),
        ("Facade", "facade"),
        ("Flyweight", "flyweight"),
        ("Proxy", "proxy"),
    ];

    let radio_group = RadioGroup::new(patterns).on_changed(|_ctx, data, _env| {
        match *data {
            "adapter" => println!("Adapter pattern selected!"),
            "bridge" => println!("Bridge pattern selected!"),
            // ... (similar lines for other patterns)
            _ => {}
        }
    });

    let label = Label::new("Select a structural design pattern:");
    Flex::column()
        .with_child(label)
        .with_spacer(8.0)
        .with_child(radio_group);
}