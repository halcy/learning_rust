use vector_macro::{*};
use auto_ops::impl_op_ex;
use auto_ops::impl_op_ex_commutative;
use std::fmt;

// Basic vector definitions
#[derive(Copy, Clone)]
pub struct TypedVec<T, const D: usize> {
    v: [T; D],
}

pub type Scalar = f32;
pub type VecN<const D: usize> = TypedVec<Scalar, D>;

// Vector types
#[derive(Copy, Clone)]
pub struct Vec2(VecN<2>);

#[derive(Copy, Clone)]
pub struct Vec3(VecN<3>);

#[derive(Copy, Clone)]
pub struct Vec4(VecN<4>);

// Constructors and swizzling + row matrix access for vectors
impl Vec2 {
    gen_constructor!(2);
    gen_swizz_funcs!(xy);
    gen_swizz_funcs!(uv);
    gen_mat_access!(1);
}
gen_display!(Vec2 1 2);

impl Vec3 {
    gen_constructor!(3);
    gen_swizz_funcs!(xyz);
    gen_swizz_funcs!(rgb);
    gen_mat_access!(1);
}
gen_display!(Vec3 1 3);

impl Vec4 {
    gen_constructor!(4);
    gen_swizz_funcs!(xyzw);
    gen_swizz_funcs!(rgba);
    gen_mat_access!(1);
}
gen_display!(Vec4 1 4);

// Basic math ops for vectors
gen_basic_ops!(Vec2 2);
gen_basic_ops!(Vec3 3);
gen_basic_ops!(Vec4 4);

// Matrix types
#[derive(Copy, Clone)]
pub struct Mat2x2(VecN<4>);

#[derive(Copy, Clone)]
pub struct Mat2x3(VecN<6>);

#[derive(Copy, Clone)]
pub struct Mat3x2(VecN<6>);

#[derive(Copy, Clone)]
pub struct Mat3x3(VecN<9>);

#[derive(Copy, Clone)]
pub struct Mat2x4(VecN<8>);

#[derive(Copy, Clone)]
pub struct Mat4x2(VecN<8>);

#[derive(Copy, Clone)]
pub struct Mat3x4(VecN<12>);

#[derive(Copy, Clone)]
pub struct Mat4x3(VecN<12>);

#[derive(Copy, Clone)]
pub struct Mat4x4(VecN<16>);

// Constructors, matrix accessors and transposition for matrices
impl Mat2x2 {
    gen_constructor!(4);
    gen_mat_access!(2);
    gen_mat_utils!(Mat2x2 2 2);
}
gen_display!(Mat2x2 2 2);

impl Mat2x3 {
    gen_constructor!(6);
    gen_mat_access!(3);
    gen_mat_utils!(Mat3x2 2 3);
}
gen_display!(Mat2x3 2 3);

impl Mat3x2 {
    gen_constructor!(6);
    gen_mat_access!(2);
    gen_mat_utils!(Mat2x3 3 2);
}
gen_display!(Mat3x2 3 2);

impl Mat3x3 {
    gen_constructor!(9);
    gen_mat_access!(3);
    gen_mat_utils!(Mat3x3 3 3);
}
gen_display!(Mat3x3 3 3);

impl Mat2x4 {
    gen_constructor!(8);
    gen_mat_access!(4);
    gen_mat_utils!(Mat4x2 2 4);
}
gen_display!(Mat2x4 2 4);

impl Mat4x2 {
    gen_constructor!(8);
    gen_mat_access!(2);
    gen_mat_utils!(Mat2x4 4 2);
}
gen_display!(Mat4x2 4 2);

impl Mat3x4 {
    gen_constructor!(12);
    gen_mat_access!(4);
    gen_mat_utils!(Mat4x3 3 4);
}
gen_display!(Mat3x4 3 4);

impl Mat4x3 {
    gen_constructor!(12);
    gen_mat_access!(3);
    gen_mat_utils!(Mat3x4 4 3);
}
gen_display!(Mat4x3 4 3);

impl Mat4x4 {
    gen_constructor!(16);
    gen_mat_access!(4);
    gen_mat_utils!(Mat4x4 4 4);
}
gen_display!(Mat4x4 4 4);

// Basic math ops for matrices
gen_basic_ops!(Mat2x2 4);
gen_basic_ops!(Mat2x3 6);
gen_basic_ops!(Mat3x2 6);
gen_basic_ops!(Mat3x3 9);
gen_basic_ops!(Mat2x4 8);
gen_basic_ops!(Mat4x2 8);
gen_basic_ops!(Mat3x4 12);
gen_basic_ops!(Mat4x3 12);
gen_basic_ops!(Mat4x4 16);

// Traits for matrix multiplication and dot products
pub trait MatMul<T, R> {
    fn matmul(&self, other: T) -> R;
}
pub trait Dot<T> {
    fn dot(&self, other: T) -> Scalar;
}

// Matrix multiplication (Vector-Vector)
gen_mat_mul!(Vec2 Vec2 Mat2x2 2 1 2 transpose);
gen_mat_mul!(Vec3 Vec3 Mat3x3 3 1 3 transpose);
gen_mat_mul!(Vec4 Vec4 Mat4x4 4 1 4 transpose);

// Matrix multiplication (Vector-Matrix / Matrix-Vector)
gen_mat_mul!(Vec2 Mat2x2 Vec2 1 2 2);
gen_mat_mul!(Vec2 Mat2x3 Vec3 1 2 3);
gen_mat_mul!(Vec2 Mat2x4 Vec4 1 2 4);

gen_mat_mul!(Vec3 Mat3x2 Vec2 1 3 2);
gen_mat_mul!(Vec3 Mat3x3 Vec3 1 3 3);
gen_mat_mul!(Vec3 Mat3x4 Vec4 1 3 4);

gen_mat_mul!(Vec4 Mat4x2 Vec2 1 4 2);
gen_mat_mul!(Vec4 Mat4x3 Vec3 1 4 3);
gen_mat_mul!(Vec4 Mat4x4 Vec4 1 4 4);

gen_mat_mul!(Mat2x2 Vec2 Vec2 1 2 2 transpose);
gen_mat_mul!(Mat2x3 Vec3 Vec2 1 3 2 transpose);
gen_mat_mul!(Mat2x4 Vec4 Vec2 1 4 2 transpose);

gen_mat_mul!(Mat3x2 Vec2 Vec3 1 2 3 transpose);
gen_mat_mul!(Mat3x3 Vec3 Vec3 1 3 3 transpose);
gen_mat_mul!(Mat3x4 Vec4 Vec3 1 4 3 transpose);

gen_mat_mul!(Mat4x2 Vec2 Vec4 1 2 4 transpose);
gen_mat_mul!(Mat4x3 Vec3 Vec4 1 3 4 transpose);
gen_mat_mul!(Mat4x4 Vec4 Vec4 1 4 4 transpose);

// Matrix multiplication (Matrix-Matrix)
gen_mat_mul!(Mat2x2 Mat2x2 Mat2x2 2 2 2);
gen_mat_mul!(Mat2x2 Mat2x3 Mat2x3 2 2 3);
gen_mat_mul!(Mat2x2 Mat2x4 Mat2x4 2 2 4);

gen_mat_mul!(Mat2x3 Mat3x2 Mat2x2 2 3 2);
gen_mat_mul!(Mat2x3 Mat3x3 Mat2x3 2 3 3);
gen_mat_mul!(Mat2x3 Mat3x4 Mat2x4 2 3 4);

gen_mat_mul!(Mat2x4 Mat4x2 Mat2x2 2 4 2);
gen_mat_mul!(Mat2x4 Mat4x3 Mat2x3 2 4 3);
gen_mat_mul!(Mat2x4 Mat4x4 Mat2x4 2 4 4);

gen_mat_mul!(Mat3x2 Mat2x2 Mat3x2 3 2 2);
gen_mat_mul!(Mat3x2 Mat2x3 Mat3x3 3 2 3);
gen_mat_mul!(Mat3x2 Mat2x4 Mat3x4 3 2 4);

gen_mat_mul!(Mat3x3 Mat3x2 Mat3x2 3 3 2);
gen_mat_mul!(Mat3x3 Mat3x3 Mat3x3 3 3 3);
gen_mat_mul!(Mat3x3 Mat3x4 Mat3x4 3 3 4);

gen_mat_mul!(Mat3x4 Mat4x2 Mat3x2 3 4 2);
gen_mat_mul!(Mat3x4 Mat4x3 Mat3x3 3 4 3);
gen_mat_mul!(Mat3x4 Mat4x4 Mat3x4 3 4 4);

gen_mat_mul!(Mat4x2 Mat2x2 Mat4x2 4 2 2);
gen_mat_mul!(Mat4x2 Mat2x3 Mat4x3 4 2 3);
gen_mat_mul!(Mat4x2 Mat2x4 Mat4x4 4 2 4);

gen_mat_mul!(Mat4x3 Mat3x2 Mat4x2 4 3 2);
gen_mat_mul!(Mat4x3 Mat3x3 Mat4x3 4 3 3);
gen_mat_mul!(Mat4x3 Mat3x4 Mat4x4 4 3 4);

gen_mat_mul!(Mat4x4 Mat4x2 Mat4x2 4 4 2);
gen_mat_mul!(Mat4x4 Mat4x3 Mat4x3 4 4 3);
gen_mat_mul!(Mat4x4 Mat4x4 Mat4x4 4 4 4);

// Dot products
gen_dot_norm!(Vec2 2);
gen_dot_norm!(Vec3 3);
gen_dot_norm!(Vec4 4);

// Cross product
impl Vec3 {
    pub fn cross(&self, b: Vec3) -> Vec3 {
        return Vec3::new(
            self.y() * b.z() - self.z() * b.y(),
            self.z() * b.x() - self.x() * b.z(),
            self.x() * b.y() - self.y() * b.x(),
        )
    }
}

// Determinant and inverse for square matrices
impl Mat2x2 {
    pub fn determinant(&self) -> Scalar {
        return self.m(0, 0) * self.m(1, 1) - self.m(1, 0) * self.m(0, 1)
    }

    pub fn inverse(&self) -> Mat2x2 {
        return Mat2x2::new(
             self.m(1, 1), -self.m(0, 1),
            -self.m(1, 0),  self.m(0, 0)
        ) / self.determinant();
    }
}

impl Mat3x3 {
    pub fn determinant(&self) -> Scalar {
        return 
             self.m(0, 0) * self.m(1, 1) * self.m(2, 2) +
            -self.m(0, 0) * self.m(2, 1) * self.m(1, 2) +
             self.m(1, 0) * self.m(2, 1) * self.m(0, 2) +
            -self.m(1, 0) * self.m(0, 1) * self.m(2, 2) +
             self.m(2, 0) * self.m(0, 1) * self.m(1, 2) +
            -self.m(2, 0) * self.m(1, 1) * self.m(0, 2);
    }

    pub fn inverse(&self) -> Mat3x3 {
        return Mat3x3::new(
             self.0.v[4] * self.0.v[8] - self.0.v[5] * self.0.v[7],
            -self.0.v[3] * self.0.v[8] + self.0.v[5] * self.0.v[6],
             self.0.v[3] * self.0.v[7] - self.0.v[4] * self.0.v[6],
            -self.0.v[1] * self.0.v[8] + self.0.v[2] * self.0.v[7],
             self.0.v[0] * self.0.v[8] - self.0.v[2] * self.0.v[6],
            -self.0.v[0] * self.0.v[7] + self.0.v[1] * self.0.v[6],
             self.0.v[1] * self.0.v[5] - self.0.v[2] * self.0.v[4],
            -self.0.v[0] * self.0.v[5] + self.0.v[2] * self.0.v[3],
             self.0.v[0] * self.0.v[4] - self.0.v[1] * self.0.v[3]
        ) / self.determinant();
    }
}    

impl Mat4x4 {
    pub fn determinant(&self) -> Scalar {
        let a0 = self.m(0, 0) * self.m(1, 1) - self.m(1, 0) * self.m(0, 1);
        let a1 = self.m(0, 0) * self.m(2, 1) - self.m(2, 0) * self.m(0, 1);
        let a2 = self.m(0, 0) * self.m(3, 1) - self.m(3, 0) * self.m(0, 1);
        let a3 = self.m(1, 0) * self.m(2, 1) - self.m(2, 0) * self.m(1, 1);
        let a4 = self.m(1, 0) * self.m(3, 1) - self.m(3, 0) * self.m(1, 1);
        let a5 = self.m(2, 0) * self.m(3, 1) - self.m(3, 0) * self.m(2, 1);
        let b0 = self.m(0, 2) * self.m(1, 3) - self.m(1, 2) * self.m(0, 3);
        let b1 = self.m(0, 2) * self.m(2, 3) - self.m(2, 2) * self.m(0, 3);
        let b2 = self.m(0, 2) * self.m(3, 3) - self.m(3, 2) * self.m(0, 3);
        let b3 = self.m(1, 2) * self.m(2, 3) - self.m(2, 2) * self.m(1, 3);
        let b4 = self.m(1, 2) * self.m(3, 3) - self.m(3, 2) * self.m(1, 3);
        let b5 = self.m(2, 2) * self.m(3, 3) - self.m(3, 2) * self.m(2, 3);
        return a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0;
    }

    pub fn inverse(&self) -> Mat4x4 {
        let a0 = self.m(0, 0) * self.m(1, 1) - self.m(1, 0) * self.m(0, 1);
        let a1 = self.m(0, 0) * self.m(2, 1) - self.m(2, 0) * self.m(0, 1);
        let a2 = self.m(0, 0) * self.m(3, 1) - self.m(3, 0) * self.m(0, 1);
        let a3 = self.m(1, 0) * self.m(2, 1) - self.m(2, 0) * self.m(1, 1);
        let a4 = self.m(1, 0) * self.m(3, 1) - self.m(3, 0) * self.m(1, 1);
        let a5 = self.m(2, 0) * self.m(3, 1) - self.m(3, 0) * self.m(2, 1);
        let b0 = self.m(0, 2) * self.m(1, 3) - self.m(1, 2) * self.m(0, 3);
        let b1 = self.m(0, 2) * self.m(2, 3) - self.m(2, 2) * self.m(0, 3);
        let b2 = self.m(0, 2) * self.m(3, 3) - self.m(3, 2) * self.m(0, 3);
        let b3 = self.m(1, 2) * self.m(2, 3) - self.m(2, 2) * self.m(1, 3);
        let b4 = self.m(1, 2) * self.m(3, 3) - self.m(3, 2) * self.m(1, 3);
        let b5 = self.m(2, 2) * self.m(3, 3) - self.m(3, 2) * self.m(2, 3);
        let det = a0 * b5 - a1 * b4 + a2 * b3 + a3 * b2 - a4 * b1 + a5 * b0;

        return Mat4x4::new(
             self.m(1, 1) * b5 - self.m(2, 1) * b4 + self.m(3, 1) * b3,
            -self.m(0, 1) * b5 + self.m(2, 1) * b2 - self.m(3, 1) * b1,
             self.m(0, 1) * b4 - self.m(1, 1) * b2 + self.m(3, 1) * b0,
            -self.m(0, 1) * b3 + self.m(1, 1) * b1 - self.m(2, 1) * b0,

            -self.m(1, 0) * b5 + self.m(2, 0) * b4 - self.m(3, 0) * b3,
             self.m(0, 0) * b5 - self.m(2, 0) * b2 + self.m(3, 0) * b1,
            -self.m(0, 0) * b4 + self.m(1, 0) * b2 - self.m(3, 0) * b0,
             self.m(0, 0) * b3 - self.m(1, 0) * b1 + self.m(2, 0) * b0,

             self.m(1, 3) * a5 - self.m(2, 3) * a4 + self.m(3, 3) * a3,
            -self.m(0, 3) * a5 + self.m(2, 3) * a2 - self.m(3, 3) * a1,
             self.m(0, 3) * a4 - self.m(1, 3) * a2 + self.m(3, 3) * a0,
            -self.m(0, 3) * a3 + self.m(1, 3) * a1 - self.m(2, 3) * a0,

            -self.m(1, 2) * a5 + self.m(2, 2) * a4 - self.m(3, 2) * a3,
             self.m(0, 2) * a5 - self.m(2, 2) * a2 + self.m(3, 2) * a1,
            -self.m(0, 2) * a4 + self.m(1, 2) * a2 - self.m(3, 2) * a0,
             self.m(0, 2) * a3 - self.m(1, 2) * a1 + self.m(2, 2) * a0
        ) / det;
    }
}
