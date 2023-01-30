use lazy_static::lazy_static;
use crate::core::rehnda_math::{Point3f, random_int_in_range, Vec3Ext, Vec3f};

const POINT_COUNT: usize = 256;
lazy_static! {
    pub static ref PERLIN: Perlin = Perlin::new();
}

pub struct Perlin {
    random_vecs: Vec<Vec3f>,
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    fn new() -> Perlin {
        let mut random_vecs = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            random_vecs.push(Vec3f::random_vec_in_range(-1.0, 1.0));
        }

        Perlin {
            random_vecs,
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point3f) -> f32 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();
        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut c= [[[Vec3f::ZERO; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let random_index = self.perm_x[((i + di) & 255) as usize] ^ self.perm_y[((j + dj) & 255) as usize] ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.random_vecs[random_index as usize];
                }
            }
        }

        perlin_interp(&c, u, v, w)
    }

    pub fn turbulence(&self, point: &Point3f, depth: usize) -> f32 {
        let mut accum = 0.0f32;
        let mut temp_point = point.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(point);
            weight *= 0.5;
            temp_point *= 2.0;
        }
        accum.abs()
    }
}

fn perlin_interp(c: &[[[Vec3f; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;
    for i in 0..2usize {
        for j in 0..2usize {
            for k in 0..2usize {
                let weight_v = Vec3f::new(u - i as f32, v - j as f32, w - k as f32);
                accum += (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu)) *
                    (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv)) *
                    (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww)) *
                    c[i][j][k].dot(weight_v);
            }
        }
    }

    accum
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
    let mut perm: [i32; POINT_COUNT] = [0; POINT_COUNT];
    for (i, el) in &mut perm.iter_mut().enumerate() {
        *el = i as i32;
    }

    for i in 0..POINT_COUNT {
        let target = random_int_in_range(0, i as i32) as usize;
        perm.swap(i, target);
    }

    perm
}
