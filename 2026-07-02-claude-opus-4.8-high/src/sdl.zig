pub const c = @cImport({
    @cInclude("SDL3/SDL.h");
});

pub const Color = struct {
    r: u8,
    g: u8,
    b: u8,
    a: u8 = 255,

    pub fn rgb(r: u8, g: u8, b: u8) Color {
        return .{ .r = r, .g = g, .b = b };
    }

    pub fn lerp(from: Color, to: Color, t: f32) Color {
        const cl = std_clamp(t);
        return .{
            .r = mix(from.r, to.r, cl),
            .g = mix(from.g, to.g, cl),
            .b = mix(from.b, to.b, cl),
            .a = mix(from.a, to.a, cl),
        };
    }
};

fn std_clamp(t: f32) f32 {
    if (t < 0) return 0;
    if (t > 1) return 1;
    return t;
}

fn mix(a: u8, b: u8, t: f32) u8 {
    const af: f32 = @floatFromInt(a);
    const bf: f32 = @floatFromInt(b);
    return @intFromFloat(af + (bf - af) * t);
}
