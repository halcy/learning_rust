use wasm_bindgen::prelude::*;
use vector_math::{*};
use image_buffer::{ImageBuffer, Color};
use std::f32::consts::PI;
use rand::Rng;
use crossbeam;
use obj_reader::{*};
use lazy_static::lazy_static;
use std::time::{SystemTime};

fn sample_unit() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

// Sampling, uniform from a hemisphere oriented towards a vector
fn sample_hemisphere_uniform(towards: Vec3) -> Vec3 {
    let u = sample_unit() * 2.0 - 1.0;
    let v = sample_unit() * 2.0 * PI;
	let w = (1.0 - u.powf(2.0)).sqrt();
    let sample = Vec3::new(w * v.cos(), w * v.sin(), u);
    if (sample & towards) < 0.0 {
        return -sample;
    }
    else {
        return sample;
    }
}

// Sampling, from a hemisphere oriented towards a vector, weighted to center
fn sample_hemisphere_weighted(towards: Vec3, weight: Scalar) -> Vec3 {
    let sample = sample_hemisphere_uniform(towards);
    return (sample + towards * weight).normalized();
}

// Shove point in direction, to avoid speckles
fn displace(v: Vec3, towards: Vec3) -> Vec3 {
    return v + towards * 0.0001;
}

trait Object {
    fn intersect(&self, o: Vec3, r: Vec3) -> Option<Hit>;
    fn shade(&self, h: Hit, cont_prob: f32) -> Color;
    fn get_bvh_skip(&self) -> i32;
}

struct Sphere {
    c: Vec3,
    r: Scalar,
    bsdf: BSDF,
    bvh_skip: i32,
}

struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    n1: Vec3,
    n2: Vec3,
    n3: Vec3,
    bsdf: BSDF,
    bvh_skip: i32,
}

struct BSDF {
    albedo: Color,
    emission: Color,
    specularity: Scalar,
    reflectivity: Scalar,
    transmittance: Scalar,
}

#[derive(Clone, Copy)]
struct Hit {
    o: Vec3,
    d: Vec3,
    dist: Scalar,
    q: Option<Vec3>,
    n: Option<Vec3>,
}

impl BSDF {
    fn calc_refraction(d: Vec3, n: Vec3, r_inside: Scalar, r_outside: Scalar) -> Vec3 {
        // Figure out whether we're going in or out and flip things accordingly
        let mut theta1 = -(d & n);
        let rr1 = r_inside;
        let rr2 = r_outside;
        let mut rn = n;
        if theta1 < 0.0 {
            rn = -rn;
            theta1 = -(d & rn);
        }
        
        // Figure out whether we have total internal reflection
        let r = rr1 / rr2;
        let theta2 = (1.0 - (r * r) * (1.0 - theta1 * theta1)).sqrt();
        if theta2 < 0.0 {
            return (d - rn * (d & rn) * 2.0).normalized();
        }
          
        // Figure out what the Fresnel equations say about what happens next
        let rs = (rr1 * theta1 - rr2 * theta2) / (rr1 * theta1 + rr2 * theta2);
        let rs = rs * rs;
        let rp = (rr1 * theta2 - rr2 * theta1) / (rr1 * theta2 + rr2 * theta1);
        let rp = rp * rp;
        let rr = (rs + rp) / 2.0;
        
        // Choose to either refract or reflect, based on fresnel coefficient
        if sample_unit() > rr {
            // Refract
            return (r * d + (r * theta1 - theta2) * rn).normalized();
        }
        else {
            // Reflect
            return (d + 2.0 * theta1 * rn).normalized();
        }
    }

    fn shade(&self, d: Vec3, q: Vec3, n: Vec3, cont_prob: f32) -> Color {
        let mut in_radiance = Vec3::new(0.0, 0.0, 0.0);
        if sample_unit() < cont_prob {
            let ray_out;
            if sample_unit() > self.reflectivity {
                // Generate diffuse ray
                ray_out = sample_hemisphere_uniform(n).normalized();
            }
            else {
                // Generate specular ray for reflection or transmission
                let axis = sample_hemisphere_weighted(n, self.specularity).normalized();

                // Reflect or transmit
                if sample_unit() > self.transmittance {
                    ray_out = (d - axis * (d & axis) * 2.0).normalized();
                }
                else {
                    // We assume that all objects float in air. could also keep track of RI in hit struct but lazy
                    ray_out = Self::calc_refraction(d, axis, 1.0, 1.5); 
                }
            }
            let new_origin = displace(q, ray_out);
            let new_prob = cont_prob * 0.95;
            in_radiance = trace(new_origin, ray_out, new_prob);
        }
        return (in_radiance + self.emission) * self.albedo;
    }
}

impl Object for Sphere {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let o_to_c = self.c - o; 
        let proj_dist = o_to_c & d; 
        let center_dist_sq = (o_to_c & o_to_c) - proj_dist.powf(2.0);
        let r_sq = self.r.powf(2.0);
        if center_dist_sq > r_sq {
            return None;
        } 
        let hit_dist = (r_sq - center_dist_sq).sqrt(); 
        let mut hit_dist_close = proj_dist - hit_dist; 
        let mut hit_dist_far = proj_dist + hit_dist; 
        if hit_dist_close > hit_dist_far {
            (hit_dist_close, hit_dist_far) = (hit_dist_far, hit_dist_close);
        }
        if hit_dist_close < 0.0 { 
            hit_dist_close = hit_dist_far;
            if hit_dist_close < 0.0 {
                return None;
            }
        } 
        let hit = Hit {
            o: o,
            d: d,
            dist: hit_dist_close,
            q: None,
            n: None,
        };
        return Some(hit);
    }

    fn shade(&self, h: Hit, cont_prob: f32) -> Color {
        let q = h.o + h.d * h.dist;
        let n = (q - self.c).normalized();
        return self.bsdf.shade(h.d, q, n, cont_prob);
    }

    fn get_bvh_skip(&self) -> i32 {
        return self.bvh_skip;
    }
}

impl Object for Triangle {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Hit> {
        let a = self.p1;
        let b = self.p2;
        let c = self.p3;

        // Figure out triangle plane
        let triangle_ab = b - a;
        let triangle_ac = c - a;
        let triangle_nn = triangle_ab.cross(triangle_ac);
        let triangle_n = (triangle_nn).normalized();
        let triangle_support = a & triangle_n;

	    // Compute intersection distance, bail if (close to) infinite or negative
        let intersection_det = triangle_n & d;
        if intersection_det.abs() <= 0.00001 {
            return None;
        }
        let intersection_dist = (triangle_support - (triangle_n & o)) / intersection_det;
        if intersection_dist <= 0.0 {
            return None;
        }

        // Compute intersection point
        let  q = o + d * intersection_dist;

        // Test inside-ness
        let triangle_bc = c - b;
        let triangle_ca = a - c;
        let triangle_aq = q - a;
        let triangle_bq = q - b;
        let triangle_cq = q - c;

        let mut bary_a = triangle_bc.cross(triangle_bq) & triangle_n;
        let mut bary_b = triangle_ca.cross(triangle_cq) & triangle_n;
        let mut bary_c = triangle_ab.cross(triangle_aq) & triangle_n;

        // Bail if on plane but outside
        if bary_a < 0.0 || bary_b < 0.0 || bary_c < 0.0 {
            return None;
        }

        // Perform barycentric interpolation of normals
        let triangle_den = triangle_nn & triangle_n;
        bary_a /= triangle_den;
        bary_b /= triangle_den;
        bary_c /= triangle_den;
        let n = (self.n1 * bary_a + self.n2 * bary_b + self.n3 * bary_c).normalized();

        let hit = Hit {
            o: o,
            d: d,
            dist: intersection_dist,
            q: Some(q),
            n: Some(n),
        };
        return Some(hit);
    }

    fn shade(&self, h: Hit, cont_prob: f32) -> Color {
        return self.bsdf.shade(h.d, h.q.unwrap(), h.n.unwrap(), cont_prob);
    }

    fn get_bvh_skip(&self) -> i32 {
        return self.bvh_skip;
    }
}

fn trace(origin: Vec3, ray: Vec3, cont_prob: f32) -> Color {
    // Do the tracing
    let mut best_object: Option<&Box<dyn Object>> = None;
    let mut best_hit = Hit{
        o: origin,
        d: ray,
        dist: f32::INFINITY,
        n: None,
        q: None,
    };
    
    let mut bvh_skip = 0;
    for obj in unsafe { SCENE.iter() } {
        if bvh_skip > 0 {
            bvh_skip -= 1;
        }
        else {
            let intersect = obj.intersect(origin, ray);
            if !intersect.is_none() {
                if obj.get_bvh_skip() == 0 {
                    let hit = intersect.unwrap();
                    if hit.dist < best_hit.dist {
                        best_hit = hit;
                        best_object = Some(&obj);
                    }
                }
            }
            else {
                bvh_skip = obj.get_bvh_skip();
            }
        }
    }
    
    // Now, shade (possibly recurse)
    let mut col = Color::new(0.2, 0.2, 0.2) * (ray & Vec3::new(0.0, 0.0, 1.0)).powf(4.0);
    if !best_object.is_none() {
        let best_object = best_object.unwrap();
        col = best_object.shade(best_hit, cont_prob)
    }
    return col;
}

fn pixel_col(pos: Vec2) -> Color {
    // Define cam
    let origin = Vec3::new(0.0, 0.0, -6.0);
    let look_at = Vec3::new(0.0, 0.0, -5.0);

    // Calculate ray
    let up = Vec3::new(0.0, 1.0, 0.0);
    let left = (origin - look_at).normalized().cross(up);
    let screen_pixel = look_at + left * pos.x() + up * pos.y();
    let ray = (screen_pixel - origin).normalized();

    // Trace some rays
    return trace(origin, ray, 1.0);
}

// Load icosahedron object
/*lazy_static! {
    static ref ICOSAHEDRON: Vec<TriData> = read_obj("icosa.obj"); // TODO learn to read?
}*/

// Update scene stored in SCENE variable
static mut SCENE: Vec<Box<dyn Object>> = Vec::<Box<dyn Object>>::new();
fn set_scene(t: Scalar) {
    unsafe {
        SCENE.clear();
    }
    for obj_idx in 0..9 {
        let t_obj = ((obj_idx as Scalar) / 9.0 + t) * PI * 2.0;
        let i_obj = (obj_idx as Scalar / 9.0) * PI * 2.0;
        let bsdf = BSDF{
            albedo: Color::new(1.0, 1.0, 1.0),
            emission: Color::new(i_obj.sin() / 2.0 + 0.5, 0.0, i_obj.cos() / 2.0 + 0.5) * 10.0,
            specularity: 25.0,
            reflectivity: 0.3,
            transmittance: 0.0,
        };
        unsafe {
            SCENE.push(Box::new(Sphere{ 
                c: Vec3::new(
                    t_obj.sin(), 
                    -(t_obj + PI/8.0).cos() / 2.0 - 0.21, 
                    t_obj.cos() / 2.0
                ) * 3.5, 
                r: 0.8,
                bsdf: bsdf,
                bvh_skip: 0,
            }));
        }
    }

    /*
    let emit_ramp = ((0.1 - (t % 0.25)).max(0.0) / 0.1).powf(5.0);
    let icosa_shift: Vec3 = Vec3::new(0.0, -0.25 + (t * 2.0 * PI).cos() * 2.0, 0.0);
    unsafe {
        SCENE.push(Box::new(Sphere{ 
            c: icosa_shift,
            r: 2.4,
            bsdf: BSDF{
                albedo: Color::new(1.0, 1.0, 1.0),
                emission: Color::new(2.0, 2.0, 2.0) * emit_ramp,
                specularity: 40.0,
                reflectivity: 1.0 - emit_ramp * 0.8,
                transmittance: 1.0,
            },
            bvh_skip: 180,
        }));
    }

    let icosa_scale: Scalar = 1.8;
    let icosa_rot: Mat3x3 = Mat3x3::new(
            (t * 2.0 * PI).cos(), 0.0, (t * 2.0 * PI).sin(),
            0.0, 1.0, 0.0,
        -(t * 2.0 * PI).sin(), 0.0, (t * 2.0 * PI).cos(),
    );
    unsafe {
        for tri_data in ICOSAHEDRON.iter() {
            SCENE.push(Box::new(Triangle{ 
                p1: (tri_data.p[0] * icosa_scale + icosa_shift) | icosa_rot,
                p2: (tri_data.p[1] * icosa_scale + icosa_shift) | icosa_rot,
                p3: (tri_data.p[2] * icosa_scale + icosa_shift) | icosa_rot,
                n1: tri_data.n[0],
                n2: tri_data.n[1],
                n3: tri_data.n[2],
                bsdf: BSDF{
                    albedo: Color::new(1.0, 1.0, 1.0),
                    emission: Color::new(2.0, 2.0, 2.0) * emit_ramp,
                    specularity: 40.0,
                    reflectivity: 1.0 - emit_ramp * 0.8,
                    transmittance: 1.0,
                },
                bvh_skip: 0,
            }));
        }
    }*/
}
// Thread render worker
fn render_slice(mut sub_buffer: ImageBuffer, samples_per_pixel: usize, full_height: usize) {
    let half_width = sub_buffer.width as Scalar / 2.0;
    let inv_aspect = full_height as Scalar / sub_buffer.width as Scalar;
    let pixel_size = Vec2::new(1.0 / sub_buffer.width as Scalar, 1.0 / full_height as Scalar);

    for y_buffer in 0..sub_buffer.height {
        let y = y_buffer + sub_buffer.first_line;
        for x in 0..sub_buffer.width {
            let mut pixel_accumulator = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let screenspace_pos = Vec2::new(
                    x as Scalar / half_width - 1.0 + (pixel_size.x() * (sample_unit() - 0.5)), 
                    y as Scalar / half_width - inv_aspect + (pixel_size.y() * (sample_unit() - 0.5))
                );
                pixel_accumulator += pixel_col(screenspace_pos);
            }
            sub_buffer.set_pixel(x, y_buffer, pixel_accumulator / samples_per_pixel as Scalar);
        }
    }
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn render_image(frame_x: i32) -> Vec<u8> {
    // Render loop
    let thread_count = 1;
    let samples_per_pixel = 100;
    let frame_count = 100;
    let frame = frame_count - frame_x - 1;
    let image_width = 320;
    let image_height = 240;

    let t = frame as Scalar / frame_count as Scalar;
    set_scene(t);
    
    let mut data_buf = ImageBuffer::alloc_data_buf(image_width, image_height);
    let mut buffer = ImageBuffer::new(image_width, image_height, &mut data_buf);
    let sub_buffers = buffer.get_split_buffers(thread_count);


    if thread_count > 1 {
        crossbeam::scope(|scope| {
            for sub_buffer in sub_buffers {
                scope.spawn(move |_| { render_slice(sub_buffer, samples_per_pixel, image_height); });
            }
        }).expect("Threading issue");
    }
    else {
        for sub_buffer in sub_buffers {
            render_slice(sub_buffer, samples_per_pixel, image_height);
        }
    }

    buffer.tonemap_aces(0.2);
    return buffer.get_rgb8_buffer();
}

