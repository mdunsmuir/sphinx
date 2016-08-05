extern crate sphinx_rustbox;

use sphinx_rustbox::*;

fn main() {
    SphinxBox::new().unwrap().eventloop(|sphinxbox, event| {
        match event {
            Event::KeyEvent(Key::Esc) => Exit,

            event => {
                sphinxbox.rustbox.clear();
                sphinxbox.rustbox.print(0, 0, Style::empty(),
                                        Color::White, Color::Blue,
                                        &format!("{:?}", event));
                sphinxbox.rustbox.present();
                DoNothing
            },
        }
    });
}
