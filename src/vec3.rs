use std::{fmt, ops};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(PartialEq)]
pub struct Vec3(Vec<f64>);

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self(vec![a, b, c])
    }

    pub fn random() -> Self {
        Self(vec![
            crate::random_double(),
            crate::random_double(),
            crate::random_double(),
        ])
    }

    pub fn random_in_range(min: &f64, max: &f64) -> Self {
        Self(vec![
            crate::random_double_in_range(min, max),
            crate::random_double_in_range(min, max),
            crate::random_double_in_range(min, max),
        ])
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in_range(&-1.0, &1.0);
            if p.len() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        unit_vector(&Vec3::random_in_unit_sphere())
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(&self) -> f64 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self(vec![0.0; 3])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Add
////////////////////////////////////////////////////////////////////////////////

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            other.x() + self.x(),
            other.y() + self.y(),
            other.z() + self.z(),
        )
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            other.x() + self.x(),
            other.y() + self.y(),
            other.z() + self.z(),
        )
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

////////////////////////////////////////////////////////////////////////////////
// Neg
////////////////////////////////////////////////////////////////////////////////

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(vec![-self.x(), -self.y(), -self.z()])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Sub
////////////////////////////////////////////////////////////////////////////////

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Mul
////////////////////////////////////////////////////////////////////////////////

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            other.x() * self.x(),
            other.y() * self.y(),
            other.z() * self.z(),
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::Mul<&f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &f64) -> Self::Output {
        Vec3::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(other.x() * self, other.y() * self, other.z() * self)
    }
}

impl ops::Mul<&Vec3> for &f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3::new(other.x() * self, other.y() * self, other.z() * self)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Div
////////////////////////////////////////////////////////////////////////////////

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self {
        (1.0 / other) * self
    }
}

impl ops::Div<&f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &f64) -> Self::Output {
        &(1.0 / other) * self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] *= 1.0 / rhs;
        self.0[1] *= 1.0 / rhs;
        self.0[2] *= 1.0 / rhs;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index
////////////////////////////////////////////////////////////////////////////////

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

////////////////////////////////////////////////////////////////////////////////
// Display
////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale = 1.0 / 100.0;

        write!(
            f,
            "{} {} {}",
            (256.0 * clamp((self.x() * scale).sqrt(), 0.0, 0.999)) as u8,
            (256.0 * clamp((self.y() * scale).sqrt(), 0.0, 0.999)) as u8,
            (256.0 * clamp((self.z() * scale).sqrt(), 0.0, 0.999)) as u8
        )?;
        Ok(())
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / &v.len()
}
