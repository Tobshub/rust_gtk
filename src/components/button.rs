use crate::gtk_button;
use gtk::prelude::*;
use gtk::Button;

// a function to create a gtk button
pub fn create_button(
    label: &str,
    margin: i32,
    click: Option<Box<dyn Fn(&Button) -> ()>>,
) -> Button {
    let button = gtk_button!(label, margin);
    if click.is_some() {
        button.connect_clicked(click.unwrap());
    }
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
    ($label: expr, $margin_block: expr, $margin_inline: expr) => {{
        let button = Button::builder()
            .label($label)
            .margin_top($margin_block)
            .margin_bottom($margin_block)
            .margin_start($margin_inline)
            .margin_end($margin_inline)
            .build();
        button
    }};
}
