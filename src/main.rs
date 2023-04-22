use beryllium::*;
use ogl33::glClearColor;

fn main() {
    let sdl = SDL::init(InitFlags::Everything).expect("Failed to start SDL");
    sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
    sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core)
        .unwrap();
    let _win = sdl
        .create_gl_window(
            "Rust GL",
            WindowPosition::Centered,
            800,
            600,
            WindowFlags::Shown,
        )
        .expect("Failed to make window & context");

    'main_loop: loop {
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => {}
            }
        }
    }
}
