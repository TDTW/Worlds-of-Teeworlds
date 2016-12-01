extern crate sdl2;

use self::sdl2::pixels::Color;

pub mod graphics {
    use graphic::sdl2::video;
    use graphic::sdl2;

    pub struct GraphicsSDL {
        window: video::Window
    }

    impl GraphicsSDL {
        pub fn new() -> Self {
            GraphicsSDL {
                window: GraphicsSDL::create_window()
            }
        }

        fn create_window() -> video::Window {
            let sdl_context = sdl2::init().unwrap();
            let video = sdl_context.video().unwrap();

            let window = video.window("Worlds of Teewords", 800, 600)
                .position_centered()
                .opengl()
                .build().unwrap();

            return window;
        }

        pub fn swap(&self) {
            self.window.gl_swap_window();
        }
    }
}
