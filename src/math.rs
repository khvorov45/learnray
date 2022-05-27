#[derive(Default, Clone, Copy)]
pub struct V2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Default, Clone, Copy)]
pub struct V4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub fn color_to_u32argb(color: V4) -> u32 {
    let result = ((color.a * 255.0) as u32) << 24
        | ((color.r * 255.0) as u32) << 16
        | ((color.g * 255.0) as u32) << 8
        | ((color.b * 255.0) as u32);
    return result;
}
