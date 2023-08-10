const std = @import("std");
const rand = std.rand;
const math = std.math;

const Viewport = @import("viewport.zig").Viewport;
const Scene = @import("viewport.zig").Scene;
const Sphere = @import("object.zig").Sphere;
const Hit = @import("object.zig").Hit;
const NO_HIT = @import("object.zig").NO_HIT;
const Ray = @import("ray.zig").Ray;
const unit_vec = @import("ray.zig").unit_vec;
const Vec3 = @Vector(3, f32);
const vec = @import("ray.zig");

pub fn rand_vec_unit() Vec3 {
    var prng = std.rand.DefaultPrng.init(@truncate(u64, @bitCast(u128, std.time.nanoTimestamp())));
    return unit_vec(blk: while (true) {
        var x = Vec3{ prng.random().float(f32), prng.random().float(f32), prng.random().float(f32) } * Vec3{ 2.0, 2.0, 2.0 } - Vec3{ 1.0, 1.0, 1.0 };
        if (x[0] * x[0] + x[1] * x[1] + x[2] * x[2] <= 1.0) {
            break :blk x;
        }
    });
}

fn diffuse(h: Hit, _: Ray) Ray {
    // println!("diff");
    var target = h.normal + rand_vec_unit();
    return Ray{ .origin = h.point, .direction = target };
}
fn metallic(h: Hit, r: Ray) Ray {
    return Ray{ .origin = h.point, .direction = vec.reflect(vec.unit_vec(r.direction), h.normal) };
}
fn metal_fuzzy03(h: Hit, r: Ray) Ray {
    return Ray{ .origin = h.point, .direction = vec.reflect(vec.unit_vec(r.direction), h.normal) + rand_vec_unit() * Vec3{ 0.3, 0.3, 0.3 } };
}
fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) Vec3 {
    var cos_theta = vec.dot_product(-uv, n);
    if (cos_theta > 1.0) {
        cos_theta = 1.0;
    }
    var r_out_perp = (uv + n * @splat(3, cos_theta)) * @splat(3, etai_over_etat);
    var r_out_parallel = n * @splat(3, -math.sqrt(math.fabs(1.0 - vec.vec_len2(r_out_perp))));
    return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, ref_idx: f32) f32 {
    // Use Schlick's approximation for reflectance.
    var r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (math.pow(f32, (1.0 - r0) * (1.0 - cosine), 5));
}

pub const Material = struct {
    metallicness: f32 = 0,
    opacity: f32 = 0,
    ir: f32 = 1.0,
    fn init(metallicness: f32, opacity: f32) Material {
        return Material{ .metallicness = metallicness, .opacity = opacity };
    }

    fn onHit(self: *Material, h: Hit, r: Ray) Ray {
        var prng = std.rand.DefaultPrng.init(@truncate(u64, @bitCast(u128, std.time.nanoTimestamp())));
        if (self.opacity > 0.0) {
            var n = h.normal;
            var front_face = true;
            if (vec.dot_product(r.direction, h.normal) > 0.0) {
                n = -h.normal;
                front_face = false;
            }
            var refraction_ratio = if (front_face) 1.0 / self.ir else self.ir;

            var unit_direction = vec.unit_vec(r.direction);
            var cos_theta = vec.dot_product(-unit_direction, n);
            if (cos_theta > 1.0) {
                cos_theta = 1.0;
            }
            var sin_theta = math.sqrt(1.0 - cos_theta * cos_theta);

            var cannot_refract = refraction_ratio * sin_theta > 1.0;
            var reflectancev = reflectance(cos_theta, refraction_ratio);
            // eprintln!("ff: {} can refract: {} ref_ratio: {}", front_face, !cannot_refract, refraction_ratio);
            var direction = if (cannot_refract or reflectancev > prng.random().float(f32))
                // eprintln!("reflect");
                vec.reflect(unit_direction, n)
            else
                // eprintln!("ud: {:?} hn: {:?}", unit_direction, n);
                refract(unit_direction, n, refraction_ratio);

            return Ray{ .origin = h.point, .direction = direction };
        }
        var sc = diffuse(h, r).direction * @splat(3, 1.0 - self.metallicness);
        var reflect = metallic(h, r);
        reflect.direction = reflect.direction * @splat(3, self.metallicness) + sc;
        return reflect;
    }
};

pub const diffuseM = Material{};
pub const metallicM = Material{ .metallicness = 1.0 };
pub const glassM = Material{ .metallicness = 1.0, .opacity = 1.0, .ir = 1.5 };
pub const glassRM = Material{ .metallicness = 1.0, .opacity = 1.0, .ir = 1 / glassM.ir };
pub const fuzzy3M = Material{ .metallicness = 0.7 };

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
        return ray_color(min_hit.mat.onHit(min_hit, r), scene, depth - 1) * min_hit.col;
    }
}

pub fn diffuseTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 900;
    const h = 600;
    const samples = 100;
    var view = Viewport{ .width = w, .height = h, .aspect_ratio = @intToFloat(f32, w) / @intToFloat(f32, h), .samples = samples };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuseM, .col = Vec3{ 1.0, 0.2, 0.2 } });
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5, .mat = diffuseM, .col = Vec3{ 0.8, 0.8, 1.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0, .mat = diffuseM, .col = Vec3{ 0.2, 1.0, 0.2 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, 0.0 }, .radius = 1.0 });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "diffuse_test.png");
}

pub fn metalTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 800;
    const h = 600;
    const samples = 100;
    var view = Viewport{ .width = w, .height = h, .aspect_ratio = @intToFloat(f32, w) / @intToFloat(f32, h), .samples = samples, .depth = 10 };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5, .mat = fuzzy3M, .col = Vec3{ 0.6, 0.6, 0.6 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5, .mat = metallicM, .col = Vec3{ 0.5, 0.9, 0.9 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, -2.0 }, .radius = 1.0, .mat = diffuseM, .col = Vec3{ 0.5, 1.0, 0.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, -1000.9, 0.0 }, .radius = 1000.0, .mat = diffuseM, .col = Vec3{ 1.0, 0.0, 1.0 } });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "metal_test.png");
}

pub fn glassTest() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const w = 800;
    const h = 600;
    const samples = 100;
    var view = Viewport{ .width = w, .height = h, .aspect_ratio = @intToFloat(f32, w) / @intToFloat(f32, h), .samples = samples, .depth = 10 };
    var spheres = try std.ArrayList(Sphere).initCapacity(gpa.allocator(), 4);
    // try spheres.append(Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5, .mat = fuzzy3M, .col = Vec3{ 0.6, 0.6, 0.6 } });
    // try spheres.append(Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5, .mat = metallicM, .col = Vec3{ 0.5, 0.9, 0.9 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, -1.0 }, .radius = 0.5, .mat = glassM, .col = Vec3{ 1.0, 1.0, 1.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, 0.0, -1.0 }, .radius = 0.35, .mat = glassRM, .col = Vec3{ 1.0, 1.0, 1.0 } });
    try spheres.append(Sphere{ .origin = Vec3{ 0.0, -100.5, -1.0 }, .radius = 100.0, .mat = diffuseM, .col = Vec3{ 1.0, 0.0, 1.0 } });

    var scene = Scene{ .spheres = spheres };
    defer scene.deinit();

    try view.Render(ray_color, scene, "glass_test.png");
}

// Sphere::new(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.6, 0.6, 0.6)), Some(&diffuseM)),
// Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.5, 0.9, 0.9)), Some(&metal)),
// Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(&diffuseM)),
// Sphere::new(Vec3 {x: 0.0, y: 0.0, z: 0.0,}, 1.0, Some(Vec3::new(0.8, 0.5, 1.0)), Some(&empty))
