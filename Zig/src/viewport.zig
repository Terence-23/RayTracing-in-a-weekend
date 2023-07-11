const zigimg = @import("zigimg");
const ray = @import("ray.zig");
const object = @import("object.zig");

const std = @import("std");
const Progress = @import("progress");

const zig_col = zigimg.color;
const rgb = zig_col.Rgb24;

const Ray = ray.Ray;
const Vec3 = ray.Vec3;
const Sphere = object.Sphere;

pub const Scene = struct {
    spheres: std.ArrayList(Sphere),

    pub fn deinit(self: *Scene) void {
        self.spheres.deinit();
    }
};

pub const Viewport = struct {
    width: u32,
    height: u32,
    aspect_ratio: f32,

    pub fn Render(self: *Viewport, comptime color_f: fn (ray: Ray, scene: Scene) rgb, scene: Scene, path: []const u8) !void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};

        var aspect_ratio: f32 = self.aspect_ratio;
        var width: u32 = self.width;
        var height: u32 = self.height;

        var viewport_height: f32 = 2.0;
        var viewport_width: f32 = aspect_ratio * viewport_height;
        var focal_length: f32 = 1.0;

        var origin: Vec3 = .{ 0.0, 0.0, 0.0 };
        var horizontal: Vec3 = .{ viewport_width, 0.0, 0.0 };
        var vertical: Vec3 = .{ 0.0, viewport_height, 0.0 };
        var lower_left_corner =
            origin - horizontal / @splat(3, @as(f32, 2.0)) - vertical / @splat(3, @as(f32, 2.0)) - Vec3{ 0.0, 0.0, focal_length };

        var stdout = std.io.getStdOut().writer();
        var pb = Progress.init(stdout);
        pb.total = height;
        pb.width = 50;
        pb.display_fraction = true;
        try stdout.writeByte('\n');

        var pix = try std.ArrayList(rgb).initCapacity(gpa.allocator(), height * width);
        defer pix.deinit();

        for (0..height) |j| {
            _ = try pb.next();
            for (0..width) |i| {
                var r = Ray{ .origin = origin, .direction = lower_left_corner +
                    horizontal * @splat(3, @intToFloat(f32, i) / @intToFloat(f32, (width - 1))) +
                    vertical * @splat(3, (@intToFloat(f32, height - 1 - j) / @intToFloat(f32, (height - 1)))) };
                var col = color_f(r, scene);
                try pix.append(col);
            }
        }
        try stdout.writeByte('\n');

        const allocator = gpa.allocator();
        var img = try zigimg.Image.create(allocator, width, height, zigimg.PixelFormat.rgb24);
        defer img.deinit();

        for (img.pixels.rgb24, pix.items) |*ptr, val| {
            ptr.* = val;
        }
        const enc_otp = zigimg.png.PNG.EncoderOptions{};
        try img.writeToFilePath(path, zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    }
};

fn ray_color(r: Ray, scene: Scene) rgb {
    const mint = 0;
    const maxt = 1000;
    var min_hit = object.NO_HIT;
    // var changed = false;
    for (scene.spheres.items) |s| {
        var hit = s.collision(r, mint, maxt);
        if (hit.equal(&object.NO_HIT)) continue;
        if (min_hit.equal(&object.NO_HIT) or hit.t < min_hit.t) {
            min_hit = hit;
            // changed = true;
        }
    }

    if (min_hit.equal(&object.NO_HIT)) {
        var unit_direction = ray.unit_vec(r.direction);
        var t = 0.5 * (unit_direction[1] + 1.0);
        return rgb{ .r = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.5)), .g = @floatToInt(u8, 255 * ((1.0 - t) + t * 0.7)), .b = 255 };
    } else {
        var n = min_hit.normal;
        return rgb{ .r = @floatToInt(u8, 255 * (0.5 * (n[0] + 1.0))), .g = @floatToInt(u8, 255 * (0.5 * (n[1] + 1.0))), .b = @floatToInt(u8, 255 * (0.5 * (n[2] + 1.0))) };
    }
}

pub fn sceneTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    var view = Viewport{ .width = 900, .height = 600, .aspect_ratio = 900.0 / 600.0 };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0 });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, 0.0 }, .radius = 1.0 });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "view_test.png");
}
