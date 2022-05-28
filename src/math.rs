use core::cmp::max;
use core::cmp::min;

#[derive(Default, Clone, Copy)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
}

impl core::ops::Add<V2i> for V2i {
    type Output = V2i;
    fn add(self, other: V2i) -> V2i {
        V2i::new(self.x + other.x, self.y + other.y)
    }
}

impl core::ops::Sub<V2i> for V2i {
    type Output = V2i;
    fn sub(self, other: V2i) -> V2i {
        V2i::new(self.x - other.x, self.y - other.y)
    }
}

impl V2i {
    pub fn new(x: i32, y: i32) -> V2i {
        V2i { x, y }
    }
}

#[derive(Default, Clone, Copy)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl core::ops::Add<V3> for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Add<f32> for V3 {
    type Output = V3;
    fn add(self, other: f32) -> V3 {
        V3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl core::ops::Add<V3> for f32 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl core::ops::Sub<V3> for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        V3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl core::ops::Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, other: f32) -> V3 {
        V3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl core::ops::Mul<V3> for f32 {
    type Output = V3;
    fn mul(self, other: V3) -> V3 {
        V3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(self) -> f32 {
        sqrtf(self.length_sq())
    }
    pub fn normalize(self) -> V3 {
        self * (1.0 / self.length())
    }
    pub fn dot(self, v2: V3) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
}

#[derive(Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl core::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl core::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
            a: self.a * other,
        }
    }
}

impl core::ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
            a: self * other.a,
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
    pub fn from_v3(v3: V3) -> Color {
        Color {
            r: v3.x,
            g: v3.y,
            b: v3.z,
            a: 1.0,
        }
    }
    pub fn to_u32argb(&self) -> u32 {
        ((self.a * 255.0) as u32) << 24
            | ((self.r * 255.0) as u32) << 16
            | ((self.g * 255.0) as u32) << 8
            | ((self.b * 255.0) as u32)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Rect2i {
    pub topleft: V2i,
    pub dim: V2i,
}

impl Rect2i {
    pub fn new(topleft: V2i, dim: V2i) -> Rect2i {
        Rect2i { topleft, dim }
    }
    pub fn bottomright(&self) -> V2i {
        self.topleft + self.dim
    }
    pub fn clip_to_rect(&self, rect: Rect2i) -> Rect2i {
        let topleft = V2i {
            x: max(self.topleft.x, rect.topleft.x),
            y: max(self.topleft.y, rect.topleft.y),
        };
        let bottomright = V2i {
            x: min(self.bottomright().x, rect.bottomright().x),
            y: min(self.bottomright().y, rect.bottomright().y),
        };
        let dim = bottomright - topleft;
        Rect2i::new(topleft, dim)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Ray {
    pub origin: V3,
    pub dir: V3,
}

impl Ray {
    pub fn at(&self, t: f32) -> V3 {
        self.origin + t * self.dir
    }
}

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub p: V3,
    /// Against the ray
    pub normal: V3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_outward(p: V3, t: f32, outward_normal: V3, ray_dir: V3) -> HitRecord {
        let front_face = ray_dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Sphere {
    pub center: V3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: V3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
    pub fn hit(self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_minus_center = ray.origin - self.center;

        let a = ray.dir.length_sq();
        let half_b = origin_minus_center.dot(ray.dir);
        let c = origin_minus_center.length_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant >= 0.0 {
            let sqrtd = sqrtf(discriminant);
            let mut root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
            }

            if root < t_min || root > t_max {
                None
            } else {
                let p = ray.at(root);
                let t = root;
                let normal = (p - self.center) * (1.0 / self.radius);
                Some(HitRecord::new_outward(p, t, normal, ray.dir))
            }
        } else {
            None
        }
    }
}

pub fn _to_radians(degrees: f32) -> f32 {
    degrees * core::f32::consts::PI / 180.0
}

//
// SECTION https://github.com/rust-lang/libm
//

macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all(feature = "unstable", $($clause)*))]
        {
            if true { // thwart the dead code lint
                $e
            }
        }
    };
}

pub fn sqrtf(x: f32) -> f32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `f32.sqrt` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return if x < 0.0 {
                ::core::f32::NAN
            } else {
                unsafe { ::core::intrinsics::sqrtf32(x) }
            }
        }
    }
    #[cfg(target_feature = "sse")]
    {
        // Note: This path is unlikely since LLVM will usually have already
        // optimized sqrt calls into hardware instructions if sse is available,
        // but if someone does end up here they'll apprected the speed increase.
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;
        unsafe {
            let m = _mm_set_ss(x);
            let m_sqrt = _mm_sqrt_ss(m);
            _mm_cvtss_f32(m_sqrt)
        }
    }
    #[cfg(not(target_feature = "sse"))]
    {
        const TINY: f32 = 1.0e-30;

        let mut z: f32;
        let sign: i32 = 0x80000000u32 as i32;
        let mut ix: i32;
        let mut s: i32;
        let mut q: i32;
        let mut m: i32;
        let mut t: i32;
        let mut i: i32;
        let mut r: u32;

        ix = x.to_bits() as i32;

        /* take care of Inf and NaN */
        if (ix as u32 & 0x7f800000) == 0x7f800000 {
            return x * x + x; /* sqrt(NaN)=NaN, sqrt(+inf)=+inf, sqrt(-inf)=sNaN */
        }

        /* take care of zero */
        if ix <= 0 {
            if (ix & !sign) == 0 {
                return x; /* sqrt(+-0) = +-0 */
            }
            if ix < 0 {
                return (x - x) / (x - x); /* sqrt(-ve) = sNaN */
            }
        }

        /* normalize x */
        m = ix >> 23;
        if m == 0 {
            /* subnormal x */
            i = 0;
            while ix & 0x00800000 == 0 {
                ix <<= 1;
                i = i + 1;
            }
            m -= i - 1;
        }
        m -= 127; /* unbias exponent */
        ix = (ix & 0x007fffff) | 0x00800000;
        if m & 1 == 1 {
            /* odd m, double x to make it even */
            ix += ix;
        }
        m >>= 1; /* m = [m/2] */

        /* generate sqrt(x) bit by bit */
        ix += ix;
        q = 0;
        s = 0;
        r = 0x01000000; /* r = moving bit from right to left */

        while r != 0 {
            t = s + r as i32;
            if t <= ix {
                s = t + r as i32;
                ix -= t;
                q += r as i32;
            }
            ix += ix;
            r >>= 1;
        }

        /* use floating add to find out rounding direction */
        if ix != 0 {
            z = 1.0 - TINY; /* raise inexact flag */
            if z >= 1.0 {
                z = 1.0 + TINY;
                if z > 1.0 {
                    q += 2;
                } else {
                    q += q & 1;
                }
            }
        }

        ix = (q >> 1) + 0x3f000000;
        ix += m << 23;
        f32::from_bits(ix as u32)
    }
}

//
// SECTION https://github.com/rust-random/rand
//

const MULTIPLIER: u64 = 6364136223846793005;

pub struct Lcg64Xsh32 {
    state: u64,
    increment: u64,
}

impl Default for Lcg64Xsh32 {
    fn default() -> Self {
        Self::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7)
    }
}

impl Lcg64Xsh32 {
    pub fn new(state: u64, stream: u64) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Lcg64Xsh32::from_state_incr(state, increment)
    }

    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Lcg64Xsh32 { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);
    }

    pub fn u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        // Output function XSH RR: xorshift high (bits), followed by a random rotate
        // Constants are for 64-bit state, 32-bit output
        const ROTATE: u32 = 59; // 64 - 5
        const XSHIFT: u32 = 18; // (5 + 32) / 2
        const SPARE: u32 = 27; // 64 - 32 - 5

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }

    pub fn f32_01(&mut self) -> f32 {
        let sign_and_exp_bits = 1 + 8;
        let mantissa_bits = 23;
        let mantissa = self.u32() << sign_and_exp_bits >> sign_and_exp_bits;
        let mantissa_f32 = mantissa as f32;
        let max_mantissa_f32 = (1 << (mantissa_bits)) as f32;
        mantissa_f32 / max_mantissa_f32
    }
}
