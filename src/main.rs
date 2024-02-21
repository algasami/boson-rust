use boson_rust::{
    engine::{BosonEngine, Object3D},
    linalg::{Vec3, ID_MAT4X4},
};
fn main() {
    println!("Hello, world!");
    let mut engine: BosonEngine<100, 20> = Default::default();
    let vertices: Vec<Vec3> = vec![
        Vec3 {
            data: [1.0, 0.0, 0.0, 1.0],
        },
        Vec3 {
            data: [0.0, 1.0, 0.0, 1.0],
        },
        Vec3 {
            data: [0.0, 0.0, 1.0, 1.0],
        },
        Vec3 {
            data: [0.0, 0.0, 0.0, 1.0],
        },
    ];
    let objects: Vec<Object3D> = vec![Object3D {
        triangles: vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]],
        model_matrix: ID_MAT4X4,
    }];
    engine.vertices = Some(&vertices);
    engine.objects = Some(&objects);
    engine.raytrace();
    engine.display();
}
