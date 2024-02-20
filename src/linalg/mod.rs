use std::{fmt, ops};

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
    pub data: [f64; 4],
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4x4 {
    pub data: [[f64; 4]; 4],
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.data[0], self.data[1], self.data[2], self.data[3]
        )
    }
}

impl fmt::Display for Mat4x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..4 {
            let res = write!(
                f,
                "|{}\t{}\t{}\t{}|{}",
                self.data[i][0],
                self.data[i][1],
                self.data[i][2],
                self.data[i][3],
                if i == 3 { ' ' } else { '\n' }
            );
            match res {
                Ok(_) => {}
                Err(t) => {
                    panic!("error during fmt mat4x4 = {}", t);
                }
            }
        }
        fmt::Result::Ok(())
    }
}

impl ops::Add<Self> for Mat4x4 {
    type Output = Self;
    fn add(self, m: Self) -> Self {
        let mut out: Self = Self {
            data: [[0.0; 4]; 4],
        };
        for i in 0..4 {
            for j in 0..4 {
                out.data[i][j] = self.data[i][j] + m.data[i][j];
            }
        }
        out
    }
}

impl ops::Mul<Self> for Mat4x4 {
    type Output = Self;
    fn mul(self, m: Self) -> Self {
        let mut out: Self = Self {
            data: [[0.0; 4]; 4],
        };
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out.data[i][k] += self.data[i][k] * m.data[k][j];
                }
            }
        }
        out
    }
}

impl ops::Mul<Vec4> for Mat4x4 {
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

impl ops::Mul<f64> for Mat4x4 {
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

impl ops::Div<f64> for Mat4x4 {
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

impl ops::Add<Self> for Vec4 {
    type Output = Self;
    fn add(self, m: Self) -> Self {
        let mut out: Self = Self { data: [0.0; 4] };
        for i in 0..4 {
            out.data[i] = self.data[i] + m.data[i];
        }
        out
    }
}

impl ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, x: f64) -> Self {
        let mut out = Self { ..self };
        for i in 0..4 {
            out.data[i] *= x;
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
