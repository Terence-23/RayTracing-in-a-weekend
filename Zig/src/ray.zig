const std = @import("std");
const sqrt = std.math.sqrt;
const zigimg = @import("zigimg");
const zig_col = zigimg.color;
const rgb = zig_col.Rgb24;
const Progress = @import("progress");

pub const Vec3 = @Vector(3, f32);

pub fn dot_product(v: Vec3, u: Vec3) f32 {
    return v[0] * u[0] + v[1] * u[1] + v[2] * u[2];
}

pub fn vec3_all(v: @Vector(3, bool)) bool {
    return v[0] and v[1] and v[2];
}

pub fn vec_to_rgb(v: Vec3) rgb {
    return rgb{ .r = @floatToInt(u8, 255 * v[0]), .g = @floatToInt(u8, 255 * v[1]), .b = @floatToInt(u8, 255 * v[2]) };
}

pub fn cross_product(v: Vec3, u: Vec3) Vec3 {
    return Vec3{
        v.y * u.z - v.z * u.y,
        v.z * u.x - v.x * u.z,
        v.x * u.y - v.y * u.x,
    };
}

pub fn vec_len(v: Vec3) f32 {
    return sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
}

pub fn unit_vec(v: Vec3) Vec3 {
    return v / @splat(3, vec_len(v));
}

pub const Ray = struct {
    origin: Vec3,
    direction: Vec3,
    pub fn at(self: *const Ray, t: f32) Vec3 {
        return self.origin + self.direction * @splat(3, t);
    }
};

fn ray_color(r: Ray) rgb {
    var unit_direction = unit_vec(r.direction);
    var t = 0.5 * (unit_direction[1] + 1.0);
    return rgb{ .r = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.5)), .g = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.7)), .b = 255 }; //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

pub fn viewport_test() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    var aspect_ratio: f32 = 3.0 / 2.0;
    var width: u32 = 600;
    var height: u32 = @floatToInt(u32, (@intToFloat(f32, width) / aspect_ratio));

    var viewport_height: f32 = 2.0;
    var viewport_width: f32 = aspect_ratio * viewport_height;
    var focal_length: f32 = 1.0;

    var origin: Vec3 = .{ 0.0, 0.0, 0.0 };
    var horizontal: Vec3 = .{ viewport_width, 0.0, 0.0 };
    var vertical: Vec3 = .{ 0.0, viewport_height, 0.0 };
    var lower_left_corner =
        origin - horizontal / @splat(3, @as(f32, 2.0)) - vertical / @splat(3, @as(f32, 2.0)) - Vec3{ 0.0, 0.0, focal_length };

    var stdout = std.io.getStdOut().writer();
    var pb = Progress.init(stdout, "");
    pb.total = height;
    pb.width = 50;
    pb.display_fraction = true;
    try stdout.writeByte('\n');

    var pix = std.ArrayList(rgb).initCapacity(gpa.allocator(), height * width) catch |err| {
        std.debug.print("{}", .{err});
        return;
    };
    defer pix.deinit();

    for (0..height) |j| {
        _ = try pb.next();
        for (0..width) |i| {
            var r = Ray{ .origin = origin, .direction = lower_left_corner + horizontal * @splat(3, @intToFloat(f32, i) / @intToFloat(f32, (width - 1))) + vertical * @splat(3, (@intToFloat(f32, j) / @intToFloat(f32, (height - 1)))) };
            try pix.append(ray_color(r));
        }
    }

    const allocator = gpa.allocator();
    var img = try zigimg.Image.create(allocator, width, height, zigimg.PixelFormat.rgb24);
    defer img.deinit();

    std.debug.print("\nimg size: {}, pix size: {}, h*w: {}\n", .{ img.pixels.rgb24.len, pix.items.len, height * width });
    for (img.pixels.rgb24, pix.items) |*ptr, val| {
        ptr.* = val;
    }

    const enc_otp = zigimg.png.PNG.EncoderOptions{};
    try img.writeToFilePath("viewport_test.png", zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    std.debug.print("Viewport_test success\n", .{});
}
