const zigimg = @import("zigimg");
const ray = @import("ray.zig");
const object = @import("object.zig");

const std = @import("std");
const Progress = @import("progress");

const zig_col = zigimg.color;
const rgb = zig_col.Rgb24;
const rand_gen = std.rand.DefaultPrng;
const pow = std.math.pow;
const isNan = std.math.isNan;

const Ray = ray.Ray;
const Vec3 = ray.Vec3;
const Sphere = object.Sphere;

pub const Scene = struct {
    spheres: std.ArrayList(Sphere),

    pub fn deinit(self: *Scene) void {
        self.spheres.deinit();
    }
};
fn gamma(vec: Vec3, inv_g: f32) Vec3 {
    return Vec3{ pow(f32, vec[0], inv_g), pow(f32, vec[1], inv_g), pow(f32, vec[2], inv_g) };
}

pub const Viewport = struct {
    width: u32,
    height: u32,
    aspect_ratio: f32,
    samples: usize,
    depth: usize = 10,
    gamma: f32 = 2,

    pub fn Render(self: *Viewport, comptime color_f: fn (ray: Ray, scene: Scene, usize) Vec3, scene: Scene, path: []const u8) !void {
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
        var inv_g = 1 / self.gamma;
        var stdout = std.io.getStdOut().writer();
        var pb = Progress.init(stdout, "Writing to disk");
        pb.total = height;
        pb.width = 50;
        pb.display_fraction = true;
        try stdout.writeByte('\n');

        std.debug.print("\nimg size: {}, w: {}, h: {}\n", .{ width * height, width, height });
        var pix = try std.ArrayList(Vec3).initCapacity(gpa.allocator(), height * width);
        defer pix.deinit();

        for (0..height) |j| {
            _ = try pb.next();
            var rnd = rand_gen.init(0);
            for (0..width) |i| {
                var col = Vec3{ 0.0, 0.0, 0.0 };
                for (0..self.samples) |_| {
                    var r = Ray{ .origin = origin, .direction = lower_left_corner +
                        horizontal * @splat(3, (@intToFloat(f32, i) + rnd.random().float(f32)) / @intToFloat(f32, (width - 1))) +
                        vertical * @splat(3, ((@intToFloat(f32, height - 1 - j) + rnd.random().float(f32)) / @intToFloat(f32, (height - 1)))) };
                    col += color_f(r, scene, self.depth);
                }
                if (isNan(col[0]) or isNan(col[1]) or isNan(col[2])) std.debug.print("NaN", .{});
                try pix.append(col / @splat(3, @intToFloat(f32, self.samples)));
            }
        }
        // try stdout.writeByte('\n');

        const allocator = gpa.allocator();
        var img = try zigimg.Image.create(allocator, width, height, zigimg.PixelFormat.rgb24);
        defer img.deinit();

        for (img.pixels.rgb24, pix.items) |*ptr, val| {
            ptr.* = ray.vec_to_rgb(gamma(val, inv_g));
        }
        const enc_otp = zigimg.png.PNG.EncoderOptions{};
        try img.writeToFilePath(path, zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    }
};

fn ray_color(r: Ray, scene: Scene, depth: usize) Vec3 {
    _ = depth;
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
        return Vec3{ (1.0 - t) + t * 0.5, ((1.0 - t) + t * 0.7), 1.0 };
    } else {
        var n = min_hit.normal;
        return Vec3{ 0.5 * (n[0] + 1.0), (0.5 * (n[1] + 1.0)), 0.5 * (n[2] + 1.0) };
    }
}

pub fn sceneTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 900;
    const h = 600;
    const samples = 100;
    var view = Viewport{ .width = w, .height = h, .aspect_ratio = @intToFloat(f32, w) / @intToFloat(f32, h), .samples = samples };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0 });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, 0.0 }, .radius = 1.0 });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "view_test.png");
}
