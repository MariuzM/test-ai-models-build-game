const font = @import("font.zig");
const sdl = @import("sdl.zig");
const c = sdl.c;
const Color = sdl.Color;
const sprites = @import("sprites.zig");
const Sprite = sprites.Sprite;

pub const Draw = struct {
    r: *c.SDL_Renderer,
    cam_x: f32 = 0,
    cam_y: f32 = 0,

    pub fn setColor(self: Draw, col: Color) void {
        _ = c.SDL_SetRenderDrawColor(self.r, col.r, col.g, col.b, col.a);
    }

    pub fn rect(self: Draw, x: f32, y: f32, w: f32, h: f32, col: Color) void {
        self.setColor(col);
        var fr = c.SDL_FRect{ .x = x, .y = y, .w = w, .h = h };
        _ = c.SDL_RenderFillRect(self.r, &fr);
    }

    pub fn wrect(self: Draw, x: f32, y: f32, w: f32, h: f32, col: Color) void {
        self.rect(x - self.cam_x, y - self.cam_y, w, h, col);
    }

    pub fn blit(self: Draw, sprite: Sprite, x: f32, y: f32, flip: bool) void {
        const width: usize = sprite[0].len;
        for (sprite, 0..) |row, ry| {
            for (row, 0..) |ch, cx| {
                const col = sprites.palette(ch) orelse continue;
                const draw_col: usize = if (flip) width - 1 - cx else cx;
                const px = x + @as(f32, @floatFromInt(draw_col)) - self.cam_x;
                const py = y + @as(f32, @floatFromInt(ry)) - self.cam_y;
                self.setColor(col);
                var fr = c.SDL_FRect{ .x = px, .y = py, .w = 1, .h = 1 };
                _ = c.SDL_RenderFillRect(self.r, &fr);
            }
        }
    }

    pub fn blitScreen(self: Draw, sprite: Sprite, x: f32, y: f32, scale: f32, tint: ?Color) void {
        for (sprite, 0..) |row, ry| {
            for (row, 0..) |ch, cx| {
                const col = tint orelse (sprites.palette(ch) orelse continue);
                if (tint != null and sprites.palette(ch) == null) continue;
                self.setColor(col);
                var fr = c.SDL_FRect{
                    .x = x + @as(f32, @floatFromInt(cx)) * scale,
                    .y = y + @as(f32, @floatFromInt(ry)) * scale,
                    .w = scale,
                    .h = scale,
                };
                _ = c.SDL_RenderFillRect(self.r, &fr);
            }
        }
    }

    pub fn text(self: Draw, str: []const u8, x: f32, y: f32, scale: f32, col: Color) f32 {
        var cursor = x;
        const advance = (font.W + 1) * scale;
        for (str) |ch| {
            if (font.glyph(ch)) |g| {
                self.setColor(col);
                for (g, 0..) |grow, ry| {
                    for (grow, 0..) |gc, cx| {
                        if (gc != '#') continue;
                        var fr = c.SDL_FRect{
                            .x = cursor + @as(f32, @floatFromInt(cx)) * scale,
                            .y = y + @as(f32, @floatFromInt(ry)) * scale,
                            .w = scale,
                            .h = scale,
                        };
                        _ = c.SDL_RenderFillRect(self.r, &fr);
                    }
                }
            }
            cursor += advance;
        }
        return cursor - x;
    }

    pub fn textWidth(str: []const u8, scale: f32) f32 {
        return @as(f32, @floatFromInt(str.len)) * (font.W + 1) * scale;
    }

    pub fn textCentered(self: Draw, str: []const u8, cx: f32, y: f32, scale: f32, col: Color) void {
        const w = textWidth(str, scale);
        const x = cx - w / 2;
        _ = self.text(str, x + scale, y + scale, scale, Color.rgb(20, 16, 28));
        _ = self.text(str, x, y, scale, col);
    }
};
