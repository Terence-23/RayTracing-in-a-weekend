const rgb_f = struct { r: f32, g: f32, b: f32 };

const rgb_u = struct { r: u8, g: u8, b: u8 };

pub fn rgb_f_to_u(col: rgb_f) rgb_u {
    return rgb_u{ .r = @as(u8, 255.999 * col.r), .g = @as(u8, 255.999 * col.g), .b = @as(u8, 255.999 * col.b) };
}
