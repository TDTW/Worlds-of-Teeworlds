mod graphic;

mod client {
    use graphic::graphics::GraphicsSDL;

    pub struct Client {
        pub running: bool,
        graphic_sdl: GraphicsSDL
    }

    impl Client {
        pub fn new() -> Self {
            Client {
                running: true,
                graphic_sdl: GraphicsSDL::new()
            }
        }

        pub fn execute(&self) {
            //self.graphic_sdl.init();

            while self.running {

                //inputUpdate
                //soundUpdate
                //windowFocus
                self.render();
            }
        }

        fn render(&self) {
            self.graphic_sdl.swap();
        }
    }
}


fn main() {
    let client = client::Client::new();
    client.execute();
}
