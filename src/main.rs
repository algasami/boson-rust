use boson_rust::{
    engine::{BosonEngine, Object3D},
    linalg::{Mat4x4, Vec3, ID_MAT4X4},
};
fn main() {
    let mut engine: BosonEngine<30, 30> = Default::default();
    let vertices: Vec<Vec3> = vec![
        Vec3 {
            data: [1.0, 0.0, 0.0],
        },
        Vec3 {
            data: [0.0, 1.0, 0.0],
        },
        Vec3 {
            data: [0.0, 0.0, 1.0],
        },
        Vec3 {
            data: [0.0, 0.0, 0.0],
        },
    ];
    let objects: Vec<Object3D> = vec![Object3D {
        triangles: vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]],
        model_matrix: Mat4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 1.2],
                [0.0, 0.0, 0.0, 1.0],
            ],
        },
    }];
    engine.vertices = Some(&vertices);
    engine.objects = Some(&objects);
    engine.view_matrix = Some(&ID_MAT4X4);
    engine.raytrace();
    engine.display();
}
