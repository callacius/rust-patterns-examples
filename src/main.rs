mod creational;
mod structural;
mod behavioral;

use fltk::prelude::WidgetBase;
use fltk::prelude::GroupExt;
use fltk::{app, button::Button, input::Input, window::Window, prelude::WidgetExt};
//use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};


fn main() {

    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 200, "Car Details");

    let mut model_input = Input::new(110, 50, 250, 25, "Model:");
    let mut year_input = Input::new(110, 90, 250, 25, "Year:");
    let mut update_button = Button::new(150, 130, 120, 30, "Update Details");

    {
        let car = creational::singleton::Carro::instance();
        model_input.set_value(&car.modelo());
        year_input.set_value(&car.ano().to_string());
    }

    
    update_button.set_callback(move |_| {
        let mut car = creational::singleton::Carro::instance();
        car.set_modelo(model_input.value());
        car.set_ano(year_input.value().to_int().unwrap());
    });

    wind.end();
    wind.show();

    app.run().unwrap();
}