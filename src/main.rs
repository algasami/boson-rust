use boson_rust::engine::BosonEngine;
fn main() {
    println!("Hello, world!");
    let mut engine: BosonEngine<100, 20> = Default::default();
    engine.raytrace();
    engine.display();
}
