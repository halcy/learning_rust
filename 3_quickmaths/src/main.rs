use vector_math::{*};
use image_buffer::{ImageBuffer, Color};
use std::f32::consts::PI;
use std::process::Command;
use rand::Rng;

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
    return v + towards * 0.00001; // TODO const this out
}

trait Object {
    fn intersect(&self, o: Vec3, r: Vec3) -> Option<Scalar>;
    fn shade(&self, r: Vec3, p: Vec3, cont_prob: f32) -> Color;
}

struct Sphere {
    c: Vec3,
    r: Scalar,
    bsdf: BSDF,
}

struct BSDF {
    albedo: Color,
    emission: Color,
    specularity: Scalar,
    reflectivity: Scalar,
    transmittance: Scalar,
}

impl BSDF {
    fn shade(&self, d: Vec3, p: Vec3, n: Vec3, cont_prob: f32) -> Color {
        let mut in_radiance = Vec3::new(0.0, 0.0, 0.0);
        if sample_unit() < cont_prob {
            let mut ray_out = Vec3::new(0.0, 0.0, 0.0);
            if sample_unit() > self.reflectivity {
                // Generate diffuse ray
                ray_out = sample_hemisphere_uniform(n).normalized();
            }
            else {
                // Generate specular ray for reflection or transmission
                let axis = sample_hemisphere_weighted(n, self.specularity).normalized();

                // For now, ignore transmission (TODO)
                ray_out = (d - axis * (d & axis) * 2.0).normalized();
            }
            let new_origin = displace(p, ray_out);
            let new_prob = cont_prob * 0.95;
            in_radiance = trace(new_origin, ray_out, new_prob);
        }
        return (in_radiance + self.emission) * self.albedo;
    }
}

impl Object for Sphere {
    fn intersect(&self, o: Vec3, d: Vec3) -> Option<Scalar> {
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
        return Some(hit_dist_close);
    }

    fn shade(&self, r: Vec3, p: Vec3, cont_prob: f32) -> Color {
        let n = (p - self.c).normalized();
        return self.bsdf.shade(r, p, n, cont_prob);
    }
}

fn trace(origin: Vec3, ray: Vec3, cont_prob: f32) -> Color {
    // Do the tracing
    let mut best_object: Option<&Box<dyn Object>> = None;
    let mut best_dist: Scalar = f32::INFINITY;
    unsafe {
        for obj in SCENE.iter() {
            let intersect = obj.intersect(origin, ray);
            if !intersect.is_none() {
                let dist = intersect.unwrap();
                if dist < best_dist {
                    best_dist = dist;
                    best_object = Some(&obj);
                }
            }
        }
    }
    
    // Now, shade (possibly recurse)
    let mut col = Color::new(0.01, 0.01, 0.01);
    if !best_object.is_none() {
        let best_object = best_object.unwrap();
        col = best_object.shade(ray, origin + ray * best_dist, cont_prob)
    }
    return col;
}

fn pixel_col(pos: Vec2, t: Scalar) -> Color {
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
            emission: Color::new(i_obj.sin() / 2.0 + 0.5, 0.0, i_obj.cos() / 2.0 + 0.5),
            specularity: 10.0,
            reflectivity: 0.3,
            transmittance: 0.0,
        };
        unsafe {
            SCENE.push(Box::new(Sphere{ 
                c: Vec3::new(
                    t_obj.sin(), 
                    -(t_obj + PI/8.0).cos() / 2.0 - 0.25, 
                    t_obj.cos() / 2.0
                ) * 3.5, 
                r: 0.8,
                bsdf: bsdf
            }));
        }

        unsafe {
            SCENE.push(Box::new(Sphere{ 
                c: Vec3::new(0.0, -0.25, 0.0),
                r: 1.2,
                bsdf: BSDF{
                    albedo: Color::new(1.0, 1.0, 1.0),
                    emission: Color::new(1.0, 1.0, 1.0),
                    specularity: 0.0,
                    reflectivity: 0.0,
                    transmittance: 0.0,
                },
            }));
        }
    }
}

fn main() {
    // Render loop
    let image_width: usize = 640;
    let image_height: usize = 480;
    let frame_count: usize = 100;
    let frame_rate: usize = 25;
    let samples_per_pixel: usize = 50;

    let half_width = image_width as Scalar / 2.0;
    let inv_aspect = image_height as Scalar / image_width as Scalar;
    let pixel_size = Vec2::new(1.0 / image_width as Scalar, 1.0 / image_height as Scalar);

    let mut buffer = ImageBuffer::new(image_width, image_height);
    for frame in 0..frame_count {
        println!("Frame {} of {}: Rendering...", frame, frame_count);

        let t = frame as Scalar / frame_count as Scalar;
        set_scene(t);

        for x in 0..image_width {
            for y in 0..image_height {
                let mut pixel_accumulator = Vec3::new(0.0, 0.0, 0.0);
                for sample in 0..samples_per_pixel {
                    let screenspace_pos = Vec2::new(
                        x as Scalar / half_width - 1.0 + pixel_size.x() * sample_unit(), 
                        y as Scalar / half_width - inv_aspect + pixel_size.y() * sample_unit()
                    );
                    pixel_accumulator += pixel_col(screenspace_pos, t);
                }
                buffer.set_pixel(x, y, pixel_accumulator / samples_per_pixel as Scalar);
            }
        }

        println!("Frame {} of {}: Writing...", frame, frame_count);
        buffer.write_bmp(&format!("out/render_{:06}.bmp", frame));
    }

    // Transform BMPs to gif
    println!("Calling ffmpeg for gif generation...");
    Command::new("ffmpeg").args([
        "-framerate", &frame_rate.to_string(),
        "-pattern_type", "glob",
        "-i", "out/render_*.bmp",
        "-f", "gif",
        "-y",
        "render.gif"
    ]).output().expect("failed to execute ffmpeg");
}

