const zigimg = @import("zigimg");
const ray = @import("ray.zig");
const object = @import("object.zig");
const viewport = @import("viewport.zig");
const materials = @import("materials.zig");

const std = @import("std");

const zig_col = zigimg.color;
const rgb = zig_col.Rgb24;

const my_json =
    \\{
    \\    "vals": {
    \\        "testing": 1,
    \\        "production": 42
    \\    },
    \\    "uptime": 9999
    \\}
;

const Config = struct {
    vals: struct {
        testing: u8,
        production: u8,
    },
    uptime: u64,
};

fn write_test() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const height: usize = 256;
    const width: usize = 256;
    var img = try zigimg.Image.create(allocator, width, height, zigimg.PixelFormat.rgb24);
    defer img.deinit();

    // var in = 0;

    var pix = std.ArrayList(rgb).initCapacity(gpa.allocator(), height * width) catch |err| {
        std.debug.print("{}", .{err});
        return;
    };
    defer pix.deinit();

    for (0..height * width) |in| {
        // const pin = in / 3;
        const col = in % width;
        const row = in / width;

        try pix.append(rgb{
            .r = @floatToInt(u8, 255 * @intToFloat(f32, col) / @intToFloat(f32, width)),
            .g = @floatToInt(u8, 255 * @intToFloat(f32, row) / @intToFloat(f32, height)),
            .b = 63,
        });
    }

    std.debug.print("img size: {}, pix size: {}, h*w: {}\n", .{ img.pixels.rgb24.len, pix.items.len, height * width });
    for (img.pixels.rgb24, pix.items) |*ptr, val| {
        ptr.* = val;
    }

    const enc_otp = zigimg.png.PNG.EncoderOptions{};
    try img.writeToFilePath("test.png", zigimg.AllFormats.ImageEncoderOptions{ .png = enc_otp });
    std.debug.print("Write_test success\n", .{});
}

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // try write_test();

    // try ray.viewport_test();

    // try object.sphere_test();

    // try object.sphere_test_normal();

    // try viewport.sceneTest();

    // try materials.diffuseTest();

    // try materials.metalTest();

    // try materials.glassTest();

    // try viewport.cameraTest();

    std.debug.print("Run `zig build test` to run the tests.\n", .{});

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};

    const Vec3 = @Vector(3, f32);

    var spheres = try std.ArrayList(object.Sphere).initCapacity(gpa.allocator(), 4);
    try spheres.append(object.Sphere{ .origin = Vec3{ 0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(object.Sphere{ .origin = Vec3{ -0.5, 0.0, -1.0 }, .radius = 0.5 });
    try spheres.append(object.Sphere{ .origin = Vec3{ 0.0, 0.2, -2.0 }, .radius = 1.0 });

    var x = viewport.Scene{ .spheres = spheres.items };
    defer spheres.deinit();
    var string = std.ArrayList(u8).init(gpa.allocator());
    try std.json.stringify(x, .{}, string.writer());

    std.log.debug("JSON: {s}", .{string.items});
    const scene = try std.json.parseFromSlice(viewport.Scene, gpa.allocator(), string.items, .{});
    std.debug.print("Scene: {}", .{scene});
}
