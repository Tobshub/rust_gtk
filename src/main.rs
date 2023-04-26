use std::cell::Cell;
use std::rc::Rc;

mod components;
use crate::components::button::create_button;

use gtk::glib::{self, clone};
use gtk::{prelude::*, Application, ApplicationWindow};

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
        Some(Box::new(clone!(@strong number => move |_| {
            number.set(number.get() + 1);
        }))),
    );
    let dec_button = create_button(
        "DECREMENT",
        12,
        Some(Box::new(move |_| {
            number.set(number.get() - 1);
        })),
    );

    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    gtk_box.append(&inc_button);
    gtk_box.append(&dec_button);

    return gtk_box;
}
