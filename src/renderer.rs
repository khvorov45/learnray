use core::cmp::min;

use crate::math::Color;
use crate::math::Rect2i;
use crate::math::V2i;
use crate::mem::Allocator;
use crate::mem::FixedArray;

#[derive(Default)]
pub struct Renderer {
    pub pixels: FixedArray<u32>,
    pub dim: V2i,
    max_dim: V2i,
}

impl Renderer {
    pub fn init<A: Allocator>(&mut self, max_width: i32, max_height: i32, allocator: &mut A) {
        if let Ok(pixels) = FixedArray::new((max_width * max_height) as usize, allocator) {
            *self = Renderer {
                pixels: pixels,
                max_dim: V2i {
                    x: max_width,
                    y: max_height,
                },
                dim: V2i { x: 0, y: 0 },
            }
        } else {
            panic!("failed to allocate pixels")
        }
    }

    pub fn clear_buffers(&mut self, width: i32, height: i32, color: Color) {
        let color32 = color.to_u32argb();

        self.dim.x = min(width, self.max_dim.x);
        self.dim.y = min(height, self.max_dim.y);

        let last_px_index = (self.dim.x * self.dim.y - 1) as usize;

        for px_index in 0..=last_px_index {
            self.pixels[px_index] = color32;
        }
    }

    pub fn draw_rect(&mut self, rect: Rect2i, color: Color) {
        let color32 = color.to_u32argb();

        let rect_clipped = rect.clip_to_rect(Rect2i::new(V2i::new(0, 0), self.dim));
        let bottomright = rect_clipped.bottomright();

        for row in rect_clipped.topleft.y..bottomright.y {
            for col in rect_clipped.topleft.x..bottomright.x {
                let px_index = (row * self.dim.x + col) as usize;
                self.pixels[px_index] = color32;
            }
        }
    }
}
