mod graphic;

mod client {
    use std::thread;
    use std::time::Duration;
    use graphic;

    trait IClient {
        fn render(&self);
    }

    pub struct Client {
        pub running: bool
    }

    impl Client {
        pub fn execute(&self) {
            graphic::init();

            while self.running {
                println!("execute in impl");
                self.render();
                thread::sleep(Duration::from_millis(1000))
            }
        }
    }

    impl IClient for Client {
        fn render(&self) {
            println!("render in impl IClient for Client");
        }
    }
}


fn main() {
    let client = client::Client {running: true};
    client.execute();
}
