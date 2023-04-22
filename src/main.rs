use std::cell::Cell;
use std::rc::Rc;

use gtk::{
    glib::{self, clone},
    Application, Button,
};
use gtk::{prelude::*, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.rust_gtk_window.tobs")
        .build();

    app.connect_activate(present_window);

    app.run()
}

fn present_window(app: &Application) {
    let gtk_box = create_gtk_box();
    let window = ApplicationWindow::builder()
        .application(app)
        // .default_width(800)
        // .default_height(600)
        .title("RUST GTK")
        .child(&gtk_box)
        .build();
    window.present();
}

fn create_gtk_box() -> gtk::Box {
    let number = Rc::new(Cell::new(0));

    let inc_button = create_button(
        "INCREMENT",
        12,
        Box::new(clone!(@strong number => move |_| {
            number.set(number.get() + 1);
        })),
    );
    let dec_button = create_button(
        "DECREMENT",
        12,
        Box::new(move |_| {
            number.set(number.get() - 1);
        }),
    );

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    gtk_box.append(&inc_button);
    gtk_box.append(&dec_button);

    return gtk_box;
}

fn create_button(label: &str, margin: i32, click: Box<dyn Fn(&Button) -> ()>) -> Button {
    let button = gtk_button!(label, margin);
    button.connect_clicked(click);
    button
}

#[macro_export]
macro_rules! gtk_button {
    ($label: expr, $margin: expr) => {{
        let button = Button::builder()
            .label($label)
            .margin_top($margin)
            .margin_bottom($margin)
            .margin_start($margin)
            .margin_end($margin)
            .build();
        button
    }};
}
