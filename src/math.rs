#[derive(Default, Clone, Copy)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
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
    pub fn length_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f32 {
        sqrtf(self.length_sq())
    }
    pub fn normalize(&self) -> V3 {
        *self * (1.0 / self.length())
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
    pub fn to_u32argb(&self) -> u32 {
        ((self.a * 255.0) as u32) << 24
            | ((self.r * 255.0) as u32) << 16
            | ((self.g * 255.0) as u32) << 8
            | ((self.b * 255.0) as u32)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Ray {
    pub orig: V3,
    pub dir: V3,
}

impl Ray {
    pub fn _at(&self, t: f32) -> V3 {
        self.orig + t * self.dir
    }
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
