const std = @import("std");

const Draw = @import("draw.zig").Draw;
const game = @import("game.zig");
const sdl = @import("sdl.zig");
const c = sdl.c;

const WIN_W = 960;
const WIN_H = 540;

pub fn main() !void {
    if (!c.SDL_Init(c.SDL_INIT_VIDEO)) {
        std.debug.print("SDL_Init failed: {s}\n", .{c.SDL_GetError()});
        return error.SDLInit;
    }
    defer c.SDL_Quit();

    const window = c.SDL_CreateWindow("Fennec Dash", WIN_W, WIN_H, c.SDL_WINDOW_RESIZABLE) orelse {
        std.debug.print("CreateWindow failed: {s}\n", .{c.SDL_GetError()});
        return error.Window;
    };
    defer c.SDL_DestroyWindow(window);

    const renderer = c.SDL_CreateRenderer(window, null) orelse {
        std.debug.print("CreateRenderer failed: {s}\n", .{c.SDL_GetError()});
        return error.Renderer;
    };
    defer c.SDL_DestroyRenderer(renderer);

    _ = c.SDL_SetRenderLogicalPresentation(renderer, @intFromFloat(game.VW), @intFromFloat(game.VH), c.SDL_LOGICAL_PRESENTATION_LETTERBOX);
    _ = c.SDL_SetRenderDrawBlendMode(renderer, c.SDL_BLENDMODE_BLEND);
    _ = c.SDL_SetRenderVSync(renderer, 1);

    var g = game.Game.init(c.SDL_GetPerformanceCounter());
    var d = Draw{ .r = renderer };

    const freq: f64 = @floatFromInt(c.SDL_GetPerformanceFrequency());
    var prev = c.SDL_GetPerformanceCounter();
    var acc: f64 = 0;
    const step: f64 = 1.0 / 60.0;

    var jump_edge = false;
    var start_edge = false;
    var restart_edge = false;

    var running = true;
    while (running) {
        var ev: c.SDL_Event = undefined;
        while (c.SDL_PollEvent(&ev)) {
            switch (ev.type) {
                c.SDL_EVENT_QUIT => running = false,
                c.SDL_EVENT_KEY_DOWN => {
                    if (ev.key.repeat) continue;
                    switch (ev.key.scancode) {
                        c.SDL_SCANCODE_ESCAPE => running = false,
                        c.SDL_SCANCODE_SPACE, c.SDL_SCANCODE_W, c.SDL_SCANCODE_UP => {
                            jump_edge = true;
                            start_edge = true;
                        },
                        c.SDL_SCANCODE_RETURN => start_edge = true,
                        c.SDL_SCANCODE_R => restart_edge = true,
                        else => {},
                    }
                },
                else => {},
            }
        }

        const ks = c.SDL_GetKeyboardState(null);
        var held = game.Input{};
        held.left = ks[c.SDL_SCANCODE_LEFT] or ks[c.SDL_SCANCODE_A];
        held.right = ks[c.SDL_SCANCODE_RIGHT] or ks[c.SDL_SCANCODE_D];
        held.jump_held = ks[c.SDL_SCANCODE_SPACE] or ks[c.SDL_SCANCODE_W] or ks[c.SDL_SCANCODE_UP];

        const now = c.SDL_GetPerformanceCounter();
        var frame: f64 = @as(f64, @floatFromInt(now - prev)) / freq;
        prev = now;
        if (frame > 0.1) frame = 0.1;
        acc += frame;

        var first = true;
        while (acc >= step) : (acc -= step) {
            var in = held;
            if (first) {
                in.jump_pressed = jump_edge;
                in.start_pressed = start_edge;
                in.restart_pressed = restart_edge;
                jump_edge = false;
                start_edge = false;
                restart_edge = false;
                first = false;
            }
            g.update(in, @floatCast(step));
        }

        g.render(&d);
        _ = c.SDL_RenderPresent(renderer);
    }
}
