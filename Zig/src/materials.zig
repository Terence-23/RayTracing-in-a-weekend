const std = @import("std");
const rand = std.rand;

const Viewport = @import("viewport.zig").Viewport;
const Scene = @import("viewport.zig").Scene;
const Sphere = @import("object.zig").Sphere;
const Hit = @import("object.zig").Hit;
const NO_HIT = @import("object.zig").NO_HIT;
const Ray = @import("ray.zig").Ray;
const unit_vec = @import("ray.zig").unit_vec;
const Vec3 = @Vector(3, f32);

pub fn rand_vec_unit() Vec3 {
    var prng = std.rand.DefaultPrng.init(@truncate(u64, @bitCast(u128, std.time.nanoTimestamp())));
    return unit_vec(blk: while (true) {
        var x = Vec3{ prng.random().float(f32), prng.random().float(f32), prng.random().float(f32) } * Vec3{ 2.0, 2.0, 2.0 } - Vec3{ 1.0, 1.0, 1.0 };
        if (x[0] * x[0] + x[1] * x[1] + x[2] * x[2] <= 1.0) {
            break :blk x;
        }
    });
}

pub fn diffuse(h: Hit) Ray {
    // println!("diff");
    var target = h.point + h.normal + rand_vec_unit();
    return Ray{ .origin = h.point, .direction = target - h.point };
}

fn ray_color(r: Ray, scene: Scene, depth: usize) Vec3 {
    if (depth <= 0) {
        return Vec3{ 0.0, 0.0, 0.0 };
    }
    const mint = 0.001;
    const maxt = 1000;
    var min_hit = NO_HIT;
    // var changed = false;
    for (scene.spheres.items) |s| {
        var hit = s.collision(r, mint, maxt);
        if (hit.equal(&NO_HIT)) continue;
        if (min_hit.equal(&NO_HIT) or hit.t < min_hit.t) {
            min_hit = hit;
            // changed = true;
        }
    }

    if (min_hit.equal(&NO_HIT)) {
        var unit_direction = unit_vec(r.direction);
        var t = 0.5 * (unit_direction[1] + 1.0);
        return Vec3{ (1.0 - t) + t * 0.5, ((1.0 - t) + t * 0.7), 1.0 };
    } else {
        return ray_color(min_hit.mat(min_hit), scene, depth - 1) * min_hit.col;
    }
}

pub fn diffuseTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 900;
    const h = 600;
    const samples = 100;
    var view = Viewport{ .width = w, .height = h, .aspect_ratio = @intToFloat(f32, w) / @intToFloat(f32, h), .samples = samples };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuse, .col = Vec3{ 1.0, 0.2, 0.2 } });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuse, .col = Vec3{ 0.8, 0.8, 1.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0, .mat = diffuse, .col = Vec3{ 0.2, 1.0, 0.2 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, 0.0 }, .radius = 1.0 });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "diffuse_test.png");
}
