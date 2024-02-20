#[derive(Copy, Clone)]
pub struct Vec4 {
    pub data: [f64; 4],
}

#[derive(Copy, Clone)]
pub struct Mat4x4 {
    pub data: [[f64; 4]; 4],
}

impl std::ops::Mul<Vec4> for Mat4x4 {
    type Output = Vec4;
    fn mul(self, v: Vec4) -> Vec4 {
        let mut out: Vec4 = Vec4 { data: [0.0; 4] };
        for i in 0..4 {
            for j in 0..4 {
                out.data[i] += self.data[i][j] * v.data[i];
            }
        }
        out
    }
}

impl std::ops::Mul<f64> for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, x: f64) -> Self {
        let mut out = Self { ..self };
        for i in 0..4 {
            for j in 0..4 {
                out.data[i][j] *= x;
            }
        }
        out
    }
}

impl std::ops::Div<f64> for Mat4x4 {
    type Output = Mat4x4;
    fn div(self, x: f64) -> Self {
        let mut out = Self { ..self };
        for i in 0..4 {
            for j in 0..4 {
                out.data[i][j] /= x;
            }
        }
        out
    }
}

// Global vars
pub static ID_MAT4X4: Mat4x4 = Mat4x4 {
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};
