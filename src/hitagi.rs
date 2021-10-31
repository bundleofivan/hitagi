pub struct Hitagi {
    port: String,
}

impl Hitagi {
    pub fn init(port: String) -> Hitagi {
        println!("hello");
        return Hitagi { port: port };
    }
}