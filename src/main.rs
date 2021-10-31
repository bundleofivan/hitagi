mod hitagi;
use hitagi::Hitagi;

fn main() {
    let server: hitagi::Hitagi = Hitagi::init("3000".to_string());
}
