use swizz_macro::{gen_swizz, gen_swizz_funcs, gen_swizz_assign, gen_elementwise, gen_scalar_right, gen_scalar_left, gen_basic_ops, gen_constructor};
use auto_ops::impl_op;
use auto_ops::impl_op_commutative;

// Basic vector definitions
#[derive(Copy, Clone)]
struct TypedVec<T, const D: usize> {
    v: [T; D],
}

type Scalar = f32;
type VecN<const D: usize> = TypedVec<Scalar, D>;

// Vector types
#[derive(Copy, Clone)]
struct Vec2(VecN<2>);

#[derive(Copy, Clone)]
struct Vec3(VecN<3>);

#[derive(Copy, Clone)]
struct Vec4(VecN<4>);

// Constructors and swizzling access for vectors
impl Vec2 {
    gen_constructor!(2);
    gen_swizz_funcs!(xy);
    gen_swizz_funcs!(uv);
}

impl Vec3 {
    gen_constructor!(3);
    gen_swizz_funcs!(xyz);
    gen_swizz_funcs!(rgb);
}

impl Vec4 {
    gen_constructor!(4);
    gen_swizz_funcs!(xyzw);
    gen_swizz_funcs!(rgba);
}

// Basic math ops for vectors
gen_basic_ops!(Vec2 2);
gen_basic_ops!(Vec3 3);
gen_basic_ops!(Vec4 4);

// Dot products
impl_op!(& |a: Vec2, b: Vec2| -> Scalar { a.0.v[0] * b.0.v[0] + a.0.v[1] * b.0.v[1] });
impl_op!(& |a: Vec3, b: Vec3| -> Scalar { a.0.v[0] * b.0.v[0] + a.0.v[1] * b.0.v[1] + a.0.v[2] * b.0.v[2] });
impl_op!(& |a: Vec4, b: Vec4| -> Scalar { a.0.v[0] * b.0.v[0] + a.0.v[1] * b.0.v[1] + a.0.v[2] * b.0.v[2] + a.0.v[3] * b.0.v[3] });

// Matrix types
#[derive(Copy, Clone)]
struct Mat2x2(VecN<4>);

#[derive(Copy, Clone)]
struct Mat2x3(VecN<6>);

#[derive(Copy, Clone)]
struct Mat3x2(VecN<6>);

#[derive(Copy, Clone)]
struct Mat3x3(VecN<9>);

#[derive(Copy, Clone)]
struct Mat2x4(VecN<8>);

#[derive(Copy, Clone)]
struct Mat4x2(VecN<8>);

#[derive(Copy, Clone)]
struct Mat3x4(VecN<12>);

#[derive(Copy, Clone)]
struct Mat4x3(VecN<12>);

#[derive(Copy, Clone)]
struct Mat4x4(VecN<16>);

// Constructors for matrices
impl Mat2x2 {
    gen_constructor!(4);
}
impl Mat2x3 {
    gen_constructor!(6);
}
impl Mat3x2 {
    gen_constructor!(6);
}
impl Mat3x3 {
    gen_constructor!(9);
}
impl Mat2x4 {
    gen_constructor!(8);
}
impl Mat4x2 {
    gen_constructor!(8);
}
impl Mat3x4 {
    gen_constructor!(12);
}
impl Mat4x3 {
    gen_constructor!(12);
}
impl Mat4x4 {
    gen_constructor!(16);
}

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

// Test this
fn main() {
    let vec_a = Vec2::new(2.0, 3.0);
    let mat_a = Mat2x3{ 0: VecN::<6>{ v: [0.0, 0.0, 0.0, 0.0, 1.0, 1.0] } };
    let mat_b = Mat2x3{ 0: VecN::<6>{ v: [0.0, 0.0, 0.0, 0.0, 1.0, 1.0] } };
    let mat_c = mat_a + mat_b;
    println!("{}", mat_c.0.v[5]);
    println!("{}", (vec_a * 3.0).xy().u());
}
