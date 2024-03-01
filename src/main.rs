use std::{f64::consts::PI, time::Instant};

use boson_rust::{
    engine::{BosonEngine, Object3D},
    linalg::{get_rotx, get_roty, Mat4x4, Vec3, ID_MAT4X4},
};

fn main() {
    /*
     referencing right values require inferred scope elevation
    */
    let mut vertices = vec![
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
    let mut objects = vec![Object3D {
        mesh: vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]],
        model_matrix: Mat4x4 {
            data: [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 3.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        },
    }];

    let mut engine: BosonEngine<30, 30> = BosonEngine {
        ibuffer: [[0.0; 30]; 30],
        vertices: &mut vertices,
        objects: &mut objects,
        view_matrix: &mut ID_MAT4X4.clone(),
    };
    let mut delta_time: f64 = 10.0;
    loop {
        let elapse = Instant::now();
        engine.raytrace();
        {
            let d = Instant::now().duration_since(elapse).as_millis();
            delta_time = d as f64 / 1000.0;
        }
        engine.display();
        println!("FPS: {}", 1.0 / delta_time);

        // modify other things here
        on_update(&mut engine, delta_time);
    }
}

fn on_update<const W: usize, const H: usize>(engine: &mut BosonEngine<W, H>, delta_time: f64) {
    engine.objects[0].model_matrix *=
        get_rotx(90.0 / 180.0 * PI * delta_time) * get_roty(90.0 / 180.0 * PI * delta_time);
    *engine.view_matrix *= get_roty(PI * delta_time);
}
