use core::cmp::min;

use crate::math::Color;
use crate::math::Rect2i;
use crate::math::V2i;
use crate::mem::Allocator;

pub struct Renderer {
    pub pixels: *mut [u32],
    pub dim: V2i,
    max_dim: V2i,
}

impl Default for Renderer {
    fn default() -> Renderer {
        Renderer {
            pixels: unsafe {
                core::slice::from_raw_parts_mut(core::ptr::null_mut(), 0) as *mut [u32]
            },
            dim: V2i::default(),
            max_dim: V2i::default(),
        }
    }
}

impl Renderer {
    pub fn init<A: Allocator>(&mut self, max_width: i32, max_height: i32, allocator: &mut A) {
        if let Ok(pixels) = allocator.make::<u32>((max_width * max_height) as usize) {
            *self = Renderer {
                pixels: pixels,
                max_dim: V2i {
                    x: max_width,
                    y: max_height,
                },
                dim: V2i { x: 0, y: 0 },
            }
        }
    }

    pub fn pixels(&mut self) -> &mut [u32] {
        unsafe { &mut *self.pixels }
    }

    pub fn clear_buffers(&mut self, width: i32, height: i32, color: Color) {
        let color32 = color.to_u32argb();

        self.dim.x = min(width, self.max_dim.x);
        self.dim.y = min(height, self.max_dim.y);

        let last_px_index = (self.dim.x * self.dim.y - 1) as usize;

        let pixels = self.pixels();
        for px_index in 0..=last_px_index {
            pixels[px_index] = color32;
        }
    }

    pub fn draw_rect(&mut self, rect: Rect2i, color: Color) {
        let color32 = color.to_u32argb();

        let rect_clipped = rect.clip_to_rect(Rect2i::new(V2i::new(0, 0), self.dim));
        let bottomright = rect_clipped.bottomright();

        for row in rect_clipped.topleft.y..bottomright.y {
            for col in rect_clipped.topleft.x..bottomright.x {
                let px_index = (row * self.dim.x + col) as usize;
                self.pixels()[px_index] = color32;
            }
        }
    }
}
