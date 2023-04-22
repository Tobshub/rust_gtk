use gtk::{glib, Application, Button};
use gtk::{prelude::*, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.rust_gtk_window.tobs")
        .build();

    app.connect_activate(present_window);

    app.run()
}

fn present_window(app: &Application) {
    fn x(b: &Button) {
        b.set_label("Hello world!")
    }
    let button = create_button("Press me!", 12, Box::new(x));
    let window = ApplicationWindow::builder()
        .application(app)
        // .default_width(800)
        // .default_height(600)
        .title("RUST GTK")
        .child(&button)
        .build();
    window.present();
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
