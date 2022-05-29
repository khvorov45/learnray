use crate::math::Color;
use crate::math::Rect2i;
use crate::math::V2i;
use crate::mem::Allocator;
use crate::mem::FixedArray;

#[derive(Default)]
pub struct Renderer {
    pub pixels: FixedArray<u32>,
    pub dim: V2i,
}

impl Renderer {
    pub fn new<A: Allocator>(dim: V2i, allocator: &mut A) -> Renderer {
        if let Ok(pixels) = FixedArray::new((dim.x * dim.y) as usize, allocator) {
            Renderer { pixels, dim }
        } else {
            panic!("failed to allocate pixels")
        }
    }

    pub fn clear_buffers(&mut self, color: Color) {
        let color32 = color.to_u32argb();
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
