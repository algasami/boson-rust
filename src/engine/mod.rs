use crate::linalg::{self, Mat4x4, Vec3};

// dev configs
// TODO: Should be exposed to users as well
const MAX_STEP_DISTANCE: f64 = 2.0;
const STEP_DISTANCE: f64 = 0.01;
const MAX_STEPS: u32 = (MAX_STEP_DISTANCE / STEP_DISTANCE) as u32;

/**
 * Boson engine
 * This is intended as a singleton (although it's never forbidden to instantiate multiple instances)
 */
pub struct BosonEngine<'a, const WIDTH: usize, const HEIGHT: usize> {
    ibuffer: [[f64; WIDTH]; HEIGHT],
    pub vertices: Option<&'a Vec<Vec3>>,
    pub objects: Option<&'a Vec<Object3D>>,
    pub view_matrix: Option<&'a Mat4x4>,
}

pub struct Object3D {
    pub triangles: Vec<[usize; 3]>,
    pub model_matrix: Mat4x4,
}

impl<'a, const W: usize, const H: usize> BosonEngine<'a, W, H> {
    /**
     * Display assumes that **ibuffer is ready to go**, so bear this in mind.
     */
    pub fn display(&self) {
        for i in 0..H {
            for j in 0..W {
                print!("{}", get_luma(self.ibuffer[i][j]));
            }
            print!("\n");
        }
    }

    pub fn raytrace(&mut self) {
        for i in 0..H {
            for j in 0..W {
                let mut current_pos = Vec3 {
                    data: [
                        ((j / W) as f64) * 2.0 - 1.0,
                        1.0 - ((i / H) as f64) * 2.0,
                        1.0,
                    ],
                };
                let ray_step = current_pos.get_unit() * STEP_DISTANCE;
                let mut s: u32 = 0;
                let mut hit = false;
                let mut unit_normal = Vec3 { data: [0.0; 3] };
                if let (Some(objects), Some(view_matrix), Some(og_verts)) =
                    (self.objects, self.view_matrix, self.vertices)
                {
                    // precompute transform
                    let mut vertices: Vec<Vec3> = og_verts.clone();
                    for obj in objects {
                        for tri in obj.triangles.as_slice() {
                            vertices[tri[0]] = *view_matrix * obj.model_matrix * og_verts[tri[0]];
                            vertices[tri[1]] = *view_matrix * obj.model_matrix * og_verts[tri[1]];
                            vertices[tri[2]] = *view_matrix * obj.model_matrix * og_verts[tri[2]];
                        }
                    }
                    while s < MAX_STEPS && !hit {
                        for obj in objects {
                            for tri in obj.triangles.as_slice() {
                                let p0 = &vertices[tri[0]];
                                let p1 = &vertices[tri[1]];
                                let p2 = &vertices[tri[2]];
                                println!("{} {} {} {}", p0, p1, p2, &current_pos);
                                let inside = linalg::check_inside(p0, p1, p2, &current_pos);
                                if inside {
                                    hit = true;
                                    unit_normal = (*p1 - *p0).cross(&(*p2 - *p0)).get_unit();
                                    break;
                                }
                            }
                            if hit {
                                break;
                            }
                        }
                        current_pos += ray_step;
                        s += 1;
                    }
                    if hit {
                        self.ibuffer[i][j] = f64::abs(unit_normal.dot(
                            &(*view_matrix
                                * Vec3 {
                                    data: [0.0, -0.5, 0.5],
                                }),
                        ));
                    }
                } else {
                    println!("self.objects/self.view_matrix/self.vertices is none, ignoring ray tracing request");
                    break;
                }
            }
        }
    }
}

impl<'a, const W: usize, const H: usize> Default for BosonEngine<'a, W, H> {
    fn default() -> BosonEngine<'a, W, H> {
        BosonEngine {
            ibuffer: [[-0.1; W]; H],
            vertices: None,
            objects: None,
            view_matrix: None,
        }
    }
}

fn get_luma(f: f64) -> char {
    if f < 0.0 {
        return ' ';
    }
    LUMA_INDEX.as_bytes()[((LUMA_INDEX.len() - 1) as f64 * f) as usize] as char
}

const LUMA_INDEX: &'static str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
