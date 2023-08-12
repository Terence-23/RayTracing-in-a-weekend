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

    origin: Vec3,
    w: Vec3,
    v: Vec3,
    u: Vec3,
    p_delta_u: Vec3,
    p_delta_v: Vec3,
    upper_left_corner: Vec3,
    msg: []const u8 = "",

    pub fn init(width: u32, aspect_ratio: f32, samples: usize, depth: usize, _gamma: f32, vfov: f32, origin: Vec3, direction: Vec3, vup: Vec3, msg: []const u8) Viewport {
        var c_origin = origin;
        var c_dir = direction;
        var c_vup = vup;
        var c_vfov = vfov;
        var w = -c_dir;
        var u = ray.unit_vec(ray.cross_product(c_vup, w));
        var v = ray.cross_product(w, u);

        var height = @floatToInt(u32, @intToFloat(f32, width) / aspect_ratio);

        var h = std.math.tan(c_vfov * std.math.pi / 360.0);

        var viewport_height = 2.0 * h;
        var viewport_width = aspect_ratio * viewport_height;

        var viewport_u = u * @splat(3, viewport_width);
        var viewport_v = -v * @splat(3, viewport_height);

        var pixel_delta_u = viewport_u / @splat(3, @intToFloat(f32, width));
        var pixel_delta_v = viewport_v / @splat(3, @intToFloat(f32, height));

        var viewport_upper_left = c_origin - w - viewport_u / Vec3{ 2.0, 2.0, 2.0 } - viewport_v / Vec3{ 2.0, 2.0, 2.0 };
        var pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * Vec3{ 0.5, 0.5, 0.5 };

        // var horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // var vertical = Vec3::new(0.0, viewport_height, 0.0);
        // var lower_left_corner =
        //     c_origin - horizontal / 2_f32 - vertical / 2_f32 - Vec3::new(0.0, 0.0, focal_length);

        return Viewport{ .samples = samples, .aspect_ratio = aspect_ratio, .width = width, .height = height, .origin = c_origin, .w = w, .v = v, .u = u, .p_delta_u = pixel_delta_u, .p_delta_v = pixel_delta_v, .upper_left_corner = pixel00_loc, .depth = depth, .gamma = _gamma, .msg = msg };
    }
    pub fn init_def(width: u32, aspect_ratio: f32, samples: usize, depth: usize, msg: []const u8) Viewport {
        return init(width, aspect_ratio, samples, depth, 2.0, 90.0, Vec3{ 0, 0, 0 }, Vec3{ 0, 0, -1 }, Vec3{ 0, 1, 0 }, msg);
    }

    pub fn Render(self: *Viewport, comptime color_f: fn (ray: Ray, scene: Scene, usize) Vec3, scene: Scene, path: []const u8) !void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};

        var inv_g = 1 / self.gamma;
        var stdout = std.io.getStdOut().writer();
        var pb = Progress.init(stdout, self.msg);
        pb.total = self.height;
        pb.width = 50;
        pb.display_fraction = true;
        try stdout.writeByte('\n');

        std.debug.print("\nimg size: {}, w: {}, h: {}\n", .{ self.width * self.height, self.width, self.height });
        var pix = try std.ArrayList(Vec3).initCapacity(gpa.allocator(), self.height * self.width);
        defer pix.deinit();

        for (0..self.height) |j| {
            _ = try pb.next();
            var rng = rand_gen.init(@truncate(u64, @bitCast(u128, std.time.nanoTimestamp())));
            for (0..self.width) |i| {
                var col = Vec3{ 0.0, 0.0, 0.0 };
                for (0..self.samples) |_| {
                    var r = Ray{ .origin = self.origin, .direction = self.upper_left_corner +
                        self.p_delta_u * @splat(3, @intToFloat(f32, i) + rng.random().float(f32)) + self.p_delta_v * @splat(3, @intToFloat(f32, j) + rng.random().float(f32)) };
                    col += color_f(r, scene, self.depth);
                }
                if (isNan(col[0]) or isNan(col[1]) or isNan(col[2])) std.debug.print("NaN", .{});
                try pix.append(col / @splat(3, @intToFloat(f32, self.samples)));
            }
        }
        // try stdout.writeByte('\n');

        const allocator = gpa.allocator();
        var img = try zigimg.Image.create(allocator, self.width, self.height, zigimg.PixelFormat.rgb24);
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
    var view = Viewport.init_def(w, @intToFloat(f32, w) / @intToFloat(f32, h), samples, 10, "Viewport test");
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0 });
    // try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, 0.0 }, .radius = 1.0 });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "view_test.png");
}

pub fn cameraTest() !void {
    const mats = @import("materials.zig");
    const diffuseM = mats.diffuseM;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 900;
    const h = 600;
    const samples = 100;
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuseM, .col = Vec3{ 1.0, 0.2, 0.2 } });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuseM, .col = Vec3{ 0.8, 0.8, 1.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0, .mat = diffuseM, .col = Vec3{ 0.2, 1.0, 0.2 } });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    var view = Viewport.init_def(w, @intToFloat(f32, w) / @intToFloat(f32, h), samples, 10, "Camera test default");
    try view.Render(mats.ray_color, scene, "Camera_default_test.png");

    view = Viewport.init(w, @intToFloat(f32, w) / @intToFloat(f32, h), samples, 10, 2.0, 120, Vec3{ 0, 0, 0 }, Vec3{ 0, 0, -1 }, Vec3{ 0, 1, 0 }, "Camera fov 120 test");
    try view.Render(mats.ray_color, scene, "Camera_fov120_test.png");
    view = Viewport.init(w, @intToFloat(f32, w) / @intToFloat(f32, h), samples, 10, 2.0, 90, Vec3{ 0, 0, 0 }, Vec3{ 0, 0, -1 }, Vec3{ 0, -1, 0 }, "Camera upside down test");
    try view.Render(mats.ray_color, scene, "Camera_upside_down_test.png");
}
