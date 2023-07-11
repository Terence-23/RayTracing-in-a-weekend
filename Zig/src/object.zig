const std = @import("std");
const sqrt = std.math.sqrt;
const zigimg = @import("zigimg");
const zig_col = zigimg.color;
const rgb = zig_col.Rgb24;
const Progress = @import("progress");
const vec = @import("ray.zig");

const Ray = vec.Ray;
const Vec3 = vec.Vec3;

pub const Hit = struct {
    t: f32,
    normal: Vec3,
    point: Vec3,
    pub fn equal(self: *const Hit, oth: *const Hit) bool {
        return vec.vec3_all(self.normal == oth.normal) and self.t == oth.t and vec.vec3_all(self.point == oth.point);
    }
};

pub const NO_HIT = Hit{ .t = -1.0, .normal = Vec3{ 0, 0, 0 }, .point = Vec3{ 0, 0, 0 } };

pub const Sphere = struct {
    origin: vec.Vec3,
    radius: f32,
    pub fn collide(self: *Sphere, r: vec.Ray) bool {
        var oc = r.origin - self.origin;
        var a = vec.dot_product(r.direction, r.direction);
        var b = 2.0 * vec.dot_product(oc, r.direction);
        var c = vec.dot_product(oc, oc) - self.radius * self.radius;
        return b * b - (4.0 * a * c) > 0.0;
    }
    pub fn collision_normal(self: *Sphere, r: Ray) Vec3 {
        var oc = r.origin - self.origin;
        var a = vec.dot_product(r.direction, r.direction);
        var b = vec.dot_product(oc, r.direction);
        var c = vec.dot_product(oc, oc) - self.radius * self.radius;
        var d = b * b - a * c;

        if (d < 0.0) {
            return Vec3{ 0.0, 0.0, 0.0 };
        }
        var x: f32 = -1;
        if (a < 0.0) {
            x = (-b + sqrt(d)) / a;
        } else {
            x = (-b - sqrt(d)) / a;
        }

        if (x < 0.0) {
            return Vec3{ 0.0, 0.0, 0.0 };
        }
        return vec.unit_vec(r.at(x) - self.origin);
    }
    pub fn collision(self: *const Sphere, r: Ray, mint: f32, maxt: f32) Hit {
        var oc = r.origin - self.origin;
        var a = vec.dot_product(r.direction, r.direction);
        var b = vec.dot_product(oc, r.direction);
        var c = vec.dot_product(oc, oc) - self.radius * self.radius;
        var d = b * b - a * c;

        if (d < 0.0) {
            return NO_HIT;
        }
        var x: f32 = -1;
        if (a < 0.0) {
            x = (-b + sqrt(d)) / a;
        } else {
            x = (-b - sqrt(d)) / a;
        }

        if (x < mint or x > maxt) {
            return NO_HIT;
        }
        return Hit{ .t = x, .normal = vec.unit_vec(r.at(x) - self.origin), .point = r.at(x) };
    }
};

fn ray_color(r: vec.Ray) rgb {
    var sphere = Sphere{ .origin = vec.Vec3{ 0.0, 0.0, -1.0 }, .radius = 0.5 };
    if (sphere.collide(r)) {
        return rgb{ .r = 255, .g = 0.0, .b = 0.0 };
    }
    var unit_direction = vec.unit_vec(r.direction);
    var t = 0.5 * (unit_direction[1] + 1.0);
    return rgb{ .r = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.5)), .g = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.7)), .b = 255 }; //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

pub fn sphere_test() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    var aspect_ratio: f32 = 3.0 / 2.0;
    var width: u32 = 600;
    var height: u32 = @floatToInt(u32, (@intToFloat(f32, width) / aspect_ratio));

    var viewport_height: f32 = 2.0;
    var viewport_width: f32 = aspect_ratio * viewport_height;
    var focal_length: f32 = 1.0;

    var origin: vec.Vec3 = .{ 0.0, 0.0, 0.0 };
    var horizontal: vec.Vec3 = .{ viewport_width, 0.0, 0.0 };
    var vertical: vec.Vec3 = .{ 0.0, viewport_height, 0.0 };
    var lower_left_corner =
        origin - horizontal / @splat(3, @as(f32, 2.0)) - vertical / @splat(3, @as(f32, 2.0)) - vec.Vec3{ 0.0, 0.0, focal_length };

    var stdout = std.io.getStdOut().writer();
    var pb = Progress.init(stdout);
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
            var r = vec.Ray{ .origin = origin, .direction = lower_left_corner + horizontal * @splat(3, @intToFloat(f32, i) / @intToFloat(f32, (width - 1))) + vertical * @splat(3, (@intToFloat(f32, j) / @intToFloat(f32, (height - 1)))) };
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
    try img.writeToFilePath("Sphere_test.png", zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    std.debug.print("Sphere_test success\n", .{});
}

fn ray_color_normal(r: Ray) rgb {
    var sphere = Sphere{
        .origin = Vec3{ 0.0, 0.0, -1.0 },
        .radius = -0.5,
    };
    var n = sphere.collision_normal(r);
    if (!vec.vec3_all(n == Vec3{ 0.0, 0.0, 0.0 })) {
        return rgb{ .r = @floatToInt(u8, 255 * 0.5 * (n[0] + 1.0)), .g = @floatToInt(u8, 255 * 0.5 * (n[1] + 1.0)), .b = @floatToInt(u8, 255 * 0.5 * (n[2] + 1.0)) };
    }

    var unit_direction = vec.unit_vec(r.direction);
    var t = 0.5 * (unit_direction[1] + 1.0);
    return rgb{ .r = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.5)), .g = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.7)), .b = 255 };
}

pub fn sphere_test_normal() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    var aspect_ratio: f32 = 3.0 / 2.0;
    var width: u32 = 600;
    var height: u32 = @floatToInt(u32, (@intToFloat(f32, width) / aspect_ratio));

    var viewport_height: f32 = 2.0;
    var viewport_width: f32 = aspect_ratio * viewport_height;
    var focal_length: f32 = 1.0;

    var origin: vec.Vec3 = .{ 0.0, 0.0, 0.0 };
    var horizontal: vec.Vec3 = .{ viewport_width, 0.0, 0.0 };
    var vertical: vec.Vec3 = .{ 0.0, viewport_height, 0.0 };
    var lower_left_corner =
        origin - horizontal / @splat(3, @as(f32, 2.0)) - vertical / @splat(3, @as(f32, 2.0)) - vec.Vec3{ 0.0, 0.0, focal_length };

    var stdout = std.io.getStdOut().writer();
    var pb = Progress.init(stdout);
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
            var r = vec.Ray{ .origin = origin, .direction = lower_left_corner + horizontal * @splat(3, @intToFloat(f32, i) / @intToFloat(f32, (width - 1))) + vertical * @splat(3, (@intToFloat(f32, height - 1 - j) / @intToFloat(f32, (height - 1)))) };
            try pix.append(ray_color_normal(r));
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
    try img.writeToFilePath("Sphere_normal_test.png", zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    std.debug.print("Sphere_test success\n", .{});
}
