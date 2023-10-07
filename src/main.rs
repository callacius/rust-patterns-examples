use druid::widget::{Button, Flex, Label, RadioGroup};
use druid::{AppLauncher, LocalizedString, Widget, WidgetExt, WindowDesc};

mod creational;
mod structural;
mod behavioral;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("Design Patterns in Rust"))
        .window_size((400.0, 200.0));

    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<()> {
    let pattern_types = vec![
        ("Creational".to_string(), "creational"),
        ("Structural".to_string(), "structural"),
        ("Behavioral".to_string(), "behavioral"),
    ];

    let radio_group = RadioGroup::new(pattern_types)
        .on_changed(|_ctx, data, _env| match *data {
            "creational" => creational::display_patterns(),
            "structural" => structural::display_patterns(),
            "behavioral" => behavioral::display_patterns(),
            _ => {}
        });

    let label = Label::new("Select a design pattern type:");
    let button = Button::new("Exit").on_click(|ctx, _, _| ctx.window().close());

    Flex::column()
        .with_child(label)
        .with_spacer(8.0)
        .with_child(radio_group)
        .with_spacer(8.0)
        .with_child(button)
}