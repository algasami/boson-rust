pub mod engine;
pub mod linalg;

#[cfg(test)]
mod tests {
    use crate::linalg;

    #[test]
    fn linalg_vec4_add() {
        let v0 = linalg::Vec3 { data: [0.0; 4] };
        let v1 = linalg::Vec3 {
            data: [0.0, 1.0, 2.0, 1.0],
        };
        assert_eq!(v0 + v1, v1);
    }

    #[test]
    fn linalg_mat4x4_vec4_mul() {
        let v0 = linalg::Vec3 {
            data: [0.0, 1.0, 2.0, 1.0],
        };
        assert_eq!(linalg::ID_MAT4X4 * 2.0 * v0, v0 * 2.0);
    }
    #[test]
    fn linalg_vec4_mag() {
        let v0 = linalg::Vec3 {
            data: [3.0, 4.0, 0.0, 1.0],
        };
        assert_eq!(v0.get_magnitude(), 5.0);
    }
    #[test]
    fn linalg_checkinside() {
        let v0 = linalg::Vec3 {
            data: [0.0, 0.0, 0.0, 1.0],
        };
        let v1 = linalg::Vec3 {
            data: [3.0, 0.0, 0.0, 1.0],
        };
        let v2 = linalg::Vec3 {
            data: [0.0, 4.0, 0.0, 1.0],
        };
        let p = linalg::Vec3 {
            data: [0.5, 0.5, 0.0, 1.0],
        };
        assert!(linalg::check_inside(&v0, &v1, &v2, &p));
    }
}
