use boson_rust::engine::BosonEngine;
fn main() {
    println!("Hello, world!");
    let engine: BosonEngine<100, 20> = Default::default();
    engine.display();
}
