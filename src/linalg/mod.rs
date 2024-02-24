use std::{fmt, ops};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub data: [f64; 3],
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4x4 {
    pub data: [[f64; 4]; 4],
}

impl Vec3 {
    /**
     * getMagnitude IGNORES W for trans-matrix reasons
     */
    pub fn get_magnitude(&self) -> f64 {
        f64::sqrt(
            self.data[0] * self.data[0] + self.data[1] * self.data[1] + self.data[2] * self.data[2],
        )
    }
    pub fn get_unit(self) -> Self {
        self / self.get_magnitude()
    }

    pub fn dot(&self, v: &Self) -> f64 {
        self.data[0] * v.data[0] + self.data[1] * v.data[1] + self.data[2] * v.data[2]
    }

    pub fn cross(&self, b: &Self) -> Self {
        Vec3 {
            data: [
                self.data[1] * b.data[2] - self.data[2] * b.data[1],
                self.data[2] * b.data[0] - self.data[0] * b.data[2],
                self.data[0] * b.data[1] - self.data[1] * b.data[0],
            ],
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.data[0], self.data[1], self.data[2])
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
                    out.data[i][j] += self.data[i][k] * m.data[k][j];
                }
            }
        }
        out
    }
}
impl PartialEq for Mat4x4 {
    fn eq(&self, other: &Self) -> bool {
        let mut same: bool = true;
        for i in 0..4 {
            for j in 0..4 {
                if self.data[i][j] != other.data[i][j] {
                    same = false;
                    break;
                }
            }
        }
        same
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let mut same: bool = true;
        for i in 0..3 {
            if self.data[i] != other.data[i] {
                same = false;
                break;
            }
        }
        same
    }
}

impl ops::Mul<Vec3> for Mat4x4 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        let mut out: Vec3 = Vec3 { data: [0.0; 3] };
        for i in 0..3 {
            for j in 0..4 {
                if j == 3 {
                    out.data[i] += self.data[i][j];
                    continue;
                }
                out.data[i] += self.data[i][j] * v.data[j];
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

// TODO: Don't copy me!
impl ops::MulAssign for Mat4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;
    fn add(self, m: Self) -> Self {
        let mut out: Self = Self { ..self };
        for i in 0..3 {
            out.data[i] += m.data[i];
        }
        out
    }
}
impl ops::Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, m: Self) -> Self {
        let mut out: Self = Self { ..self };
        for i in 0..3 {
            out.data[i] -= m.data[i];
        }
        out
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
            ],
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, x: f64) -> Self {
        let mut out = Self { ..self };
        for i in 0..3 {
            out.data[i] *= x;
        }
        out
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, x: f64) -> Self {
        let mut out = Self { ..self };
        for i in 0..3 {
            out.data[i] /= x;
        }
        out
    }
}

const EPSILON: f64 = 0.1;
pub fn check_inside(p0: &Vec3, p1: &Vec3, p2: &Vec3, p: &Vec3) -> bool {
    let mut n1 = (*p0 - *p).cross(&(*p1 - *p));
    let mut n2 = (*p1 - *p).cross(&(*p2 - *p));
    let mut n3 = (*p2 - *p).cross(&(*p0 - *p));

    if n1.get_magnitude() <= 0.0 || n2.get_magnitude() <= 0.0 || n3.get_magnitude() <= 0.0 {
        return true;
    }
    n1 = n1.get_unit();
    n2 = n2.get_unit();
    n3 = n3.get_unit();
    n1.dot(&n2) > 1.0 - EPSILON && n2.dot(&n3) > 1.0 - EPSILON
}
// Global vars
pub const ID_MAT4X4: Mat4x4 = Mat4x4 {
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

pub fn get_rotx(radian: f64) -> Mat4x4 {
    Mat4x4 {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f64::cos(radian), -f64::sin(radian), 0.0],
            [0.0, f64::sin(radian), f64::cos(radian), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn get_roty(radian: f64) -> Mat4x4 {
    Mat4x4 {
        data: [
            [f64::cos(radian), 0.0, f64::sin(radian), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-f64::sin(radian), 0.0, f64::cos(radian), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn get_rotz(radian: f64) -> Mat4x4 {
    Mat4x4 {
        data: [
            [f64::cos(radian), -f64::sin(radian), 0.0, 0.0],
            [f64::sin(radian), f64::cos(radian), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
