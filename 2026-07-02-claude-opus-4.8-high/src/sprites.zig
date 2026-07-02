const sdl = @import("sdl.zig");
const Color = sdl.Color;

pub const Sprite = []const []const u8;

pub fn palette(ch: u8) ?Color {
    return switch (ch) {
        'K', 'e' => Color.rgb(28, 22, 34),
        'o' => Color.rgb(236, 142, 72),
        'd' => Color.rgb(198, 98, 50),
        'c' => Color.rgb(250, 224, 176),
        'w' => Color.rgb(252, 250, 244),
        'p' => Color.rgb(150, 102, 176),
        'P' => Color.rgb(80, 52, 108),
        'r' => Color.rgb(234, 74, 74),
        'm' => Color.rgb(44, 30, 58),
        'y' => Color.rgb(226, 158, 40),
        'Y' => Color.rgb(255, 214, 92),
        else => null,
    };
}

pub const fox_body: Sprite = &.{
    ".........dd..",
    "..d......odo.",
    ".ddd....oooo.",
    ".dcdd..ooeooo",
    ".ddd..oooooKc",
    ".dd.ooocccccc",
    "..d.ooooocccc",
    "....ooooooccc",
    "....cccccccc.",
};

pub const legs_idle: Sprite = &.{
    "....cc..cc...",
    "....KK..KK...",
};

pub const legs_run1: Sprite = &.{
    "...cc.....cc.",
    "...KK.....KK.",
};

pub const legs_run2: Sprite = &.{
    ".....cc.cc...",
    ".....KK.KK...",
};

pub const legs_jump: Sprite = &.{
    "...c......c..",
    "...cc....KK..",
};

pub const beetle: Sprite = &.{
    "...ppppp...",
    "..ppppppp..",
    ".ppppppppp.",
    ".ppppppprp.",
    ".PPPPPPPPP.",
    "..m.m.m.m..",
    "..m.....m..",
};

pub const heart: Sprite = &.{
    ".rr.rr.",
    "rrrrrrr",
    "rrrrrrr",
    ".rrrrr.",
    "..rrr..",
    "...r...",
};

pub const gem: Sprite = &.{
    "....y....",
    "...yYy...",
    "..yYYYy..",
    ".yYYYYYy.",
    "yYYwYYYYy",
    ".yYYYYYy.",
    "..yYYYy..",
    "...yYy...",
    "....y....",
};
