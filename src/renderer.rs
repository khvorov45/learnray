use core::cmp::min;

use crate::math::Color;
use crate::math::V2i;
use crate::mem::Allocator;

#[derive(Default)]
pub struct Renderer<'a> {
    pub pixels: &'a mut [u32],
    pub dim: V2i,
    max_dim: V2i,
}

impl<'a> Renderer<'a> {
    pub fn init<A: Allocator>(&mut self, max_width: i32, max_height: i32, allocator: &'a mut A) {
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

    pub fn clear_buffers(&mut self, width: i32, height: i32, color: Color) {
        let color32 = color.to_u32argb();

        self.dim.x = min(width, self.max_dim.x);
        self.dim.y = min(height, self.max_dim.y);

        let last_px_index = (self.dim.x * self.dim.y - 1) as usize;
        for px_index in 0..=last_px_index {
            self.pixels[px_index] = color32;
        }
    }
}
