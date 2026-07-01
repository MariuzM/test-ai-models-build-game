use macroquad::prelude::*;

fn canvas(w: u16, h: u16) -> Image {
    Image::gen_image_color(w, h, Color::from_rgba(0, 0, 0, 0))
}

fn px(img: &mut Image, x: i32, y: i32, c: Color) {
    if x < 0 || y < 0 || x >= img.width as i32 || y >= img.height as i32 {
        return;
    }
    img.set_pixel(x as u32, y as u32, c);
}

fn rect(img: &mut Image, x: i32, y: i32, w: i32, h: i32, c: Color) {
    for yy in y..y + h {
        for xx in x..x + w {
            px(img, xx, yy, c);
        }
    }
}

fn to_tex(img: Image) -> Texture2D {
    let t = Texture2D::from_image(&img);
    t.set_filter(FilterMode::Nearest);
    t
}

// Palette
const HELMET: Color = Color::from_rgba(160, 172, 192, 255);
const HELMET_HI: Color = Color::from_rgba(205, 214, 228, 255);
const PLUME: Color = Color::from_rgba(210, 64, 64, 255);
const SKIN: Color = Color::from_rgba(232, 190, 150, 255);
const ARMOR: Color = Color::from_rgba(66, 98, 168, 255);
const ARMOR_HI: Color = Color::from_rgba(104, 140, 200, 255);
const BELT: Color = Color::from_rgba(224, 178, 66, 255);
const LEG: Color = Color::from_rgba(56, 76, 132, 255);
const BOOT: Color = Color::from_rgba(34, 32, 46, 255);
const CAPE: Color = Color::from_rgba(176, 46, 52, 255);
const CAPE_DARK: Color = Color::from_rgba(132, 30, 38, 255);
const EYE: Color = Color::from_rgba(24, 22, 30, 255);

/// Draws the knight centered on a 20x22 canvas, facing right.
/// `leg_shift` moves the front/back leg for the walk cycle, `cape_flow`
/// extends the trailing cape, `crouch` tucks the legs up for jump/fall poses.
fn draw_knight(leg_shift: i32, cape_flow: i32, crouch: bool) -> Image {
    let mut img = canvas(20, 22);

    // Cape trailing behind (to the left)
    rect(&mut img, 2, 7, 3, 8, CAPE_DARK);
    rect(&mut img, 1 - cape_flow.min(2), 12, 3, 4, CAPE);
    rect(&mut img, 2, 6, 2, 1, CAPE);

    if crouch {
        // Tucked legs for jump / fall pose
        rect(&mut img, 7, 15, 3, 4, LEG);
        rect(&mut img, 11, 15, 3, 4, LEG);
        rect(&mut img, 7, 18, 3, 2, BOOT);
        rect(&mut img, 11, 18, 3, 2, BOOT);
    } else {
        let l1 = 15 + leg_shift.max(0);
        let l2 = 15 + (-leg_shift).max(0);
        rect(&mut img, 7, 15, 3, (l1 - 15 + 5).max(3), LEG);
        rect(&mut img, 11, 15, 3, (l2 - 15 + 5).max(3), LEG);
        rect(&mut img, 7, l1 + 3, 3, 2, BOOT);
        rect(&mut img, 11, l2 + 3, 3, 2, BOOT);
    }

    // Torso
    rect(&mut img, 6, 9, 9, 6, ARMOR);
    rect(&mut img, 7, 10, 3, 2, ARMOR_HI);
    rect(&mut img, 6, 13, 9, 1, BELT);
    // Front arm
    rect(&mut img, 14, 10, 2, 4, ARMOR);

    // Head
    rect(&mut img, 8, 5, 5, 4, SKIN);
    rect(&mut img, 7, 2, 7, 4, HELMET);
    rect(&mut img, 8, 2, 3, 1, HELMET_HI);
    rect(&mut img, 12, 5, 1, 1, EYE);
    rect(&mut img, 9, 1, 2, 1, PLUME);

    img
}

pub struct PlayerSprites {
    pub idle: Texture2D,
    pub run: [Texture2D; 4],
    pub jump: Texture2D,
    pub fall: Texture2D,
}

impl PlayerSprites {
    pub fn generate() -> Self {
        Self {
            idle: to_tex(draw_knight(0, 0, false)),
            run: [
                to_tex(draw_knight(2, 0, false)),
                to_tex(draw_knight(0, 1, false)),
                to_tex(draw_knight(-2, 2, false)),
                to_tex(draw_knight(0, 1, false)),
            ],
            jump: to_tex(draw_knight(0, 2, true)),
            fall: to_tex(draw_knight(0, 1, true)),
        }
    }
}

pub struct BatSprites {
    pub frames: [Texture2D; 2],
}

impl BatSprites {
    pub fn generate() -> Self {
        let body = Color::from_rgba(70, 44, 92, 255);
        let wing = Color::from_rgba(52, 32, 74, 255);
        let eye = Color::from_rgba(220, 60, 60, 255);

        let mut up = canvas(18, 12);
        rect(&mut up, 7, 4, 4, 4, body);
        rect(&mut up, 8, 5, 1, 1, eye);
        rect(&mut up, 9, 5, 1, 1, eye);
        // wings swept upward
        rect(&mut up, 4, 5, 3, 2, wing);
        rect(&mut up, 2, 3, 3, 2, wing);
        rect(&mut up, 0, 1, 3, 2, wing);
        rect(&mut up, 11, 5, 3, 2, wing);
        rect(&mut up, 13, 3, 3, 2, wing);
        rect(&mut up, 15, 1, 3, 2, wing);

        let mut down = canvas(18, 12);
        rect(&mut down, 7, 4, 4, 4, body);
        rect(&mut down, 8, 5, 1, 1, eye);
        rect(&mut down, 9, 5, 1, 1, eye);
        rect(&mut down, 4, 6, 3, 2, wing);
        rect(&mut down, 2, 8, 3, 2, wing);
        rect(&mut down, 0, 9, 3, 2, wing);
        rect(&mut down, 11, 6, 3, 2, wing);
        rect(&mut down, 13, 8, 3, 2, wing);
        rect(&mut down, 15, 9, 3, 2, wing);

        Self {
            frames: [to_tex(up), to_tex(down)],
        }
    }
}

pub struct SlimeSprites {
    pub frames: [Texture2D; 2],
}

impl SlimeSprites {
    pub fn generate() -> Self {
        let body = Color::from_rgba(74, 182, 96, 255);
        let hi = Color::from_rgba(130, 220, 140, 255);
        let eye = Color::from_rgba(20, 30, 24, 255);

        // Tall idle pose
        let mut a = canvas(18, 12);
        rect(&mut a, 6, 2, 6, 2, body);
        rect(&mut a, 4, 4, 10, 3, body);
        rect(&mut a, 2, 7, 14, 4, body);
        rect(&mut a, 5, 4, 3, 2, hi);
        rect(&mut a, 6, 8, 1, 1, eye);
        rect(&mut a, 11, 8, 1, 1, eye);

        // Squashed hop pose (wider, shorter)
        let mut b = canvas(18, 12);
        rect(&mut b, 5, 5, 8, 2, body);
        rect(&mut b, 2, 7, 14, 4, body);
        rect(&mut b, 0, 9, 18, 2, body);
        rect(&mut b, 5, 5, 3, 2, hi);
        rect(&mut b, 6, 8, 1, 1, eye);
        rect(&mut b, 11, 8, 1, 1, eye);

        Self {
            frames: [to_tex(a), to_tex(b)],
        }
    }
}

pub fn generate_gem(frame: u32) -> Texture2D {
    let mut img = canvas(12, 12);
    let base = Color::from_rgba(72, 210, 226, 255);
    let dark = Color::from_rgba(40, 150, 190, 255);
    let hi = Color::from_rgba(200, 245, 250, 255);

    let widths = [2, 4, 6, 8, 10, 8, 6, 4, 2];
    for (row, w) in widths.iter().enumerate() {
        let x0 = 6 - w / 2;
        rect(&mut img, x0, row as i32 + 1, *w, 1, base);
    }
    rect(&mut img, 3, 3, 2, 2, hi);
    rect(&mut img, 5, 9, 2, 1, dark);
    if frame % 2 == 0 {
        px(&mut img, 9, 2, hi);
        px(&mut img, 2, 6, hi);
    }
    to_tex(img)
}

pub fn generate_ground_tile(variant: u32) -> Texture2D {
    let mut img = canvas(32, 32);
    let base = Color::from_rgba(58, 52, 68, 255);
    let mortar = Color::from_rgba(38, 34, 46, 255);
    let hi = Color::from_rgba(78, 70, 88, 255);

    rect(&mut img, 0, 0, 32, 32, base);
    for y in (0..32).step_by(8) {
        rect(&mut img, 0, y, 32, 1, mortar);
    }
    let offset = if (variant % 2) == 0 { 0 } else { 8 };
    for row in 0..4 {
        let y = row * 8;
        let mut x = -offset + row % 2 * 8;
        while x < 32 {
            rect(&mut img, x, y, 1, 8, mortar);
            x += 16;
        }
    }
    rect(&mut img, 2, 1, 4, 1, hi);
    rect(&mut img, 18, 9, 3, 1, hi);
    rect(&mut img, 10, 17, 3, 1, hi);
    to_tex(img)
}

pub fn generate_spike() -> Texture2D {
    let mut img = canvas(32, 32);
    let body = Color::from_rgba(150, 150, 162, 255);
    let dark = Color::from_rgba(90, 90, 102, 255);

    for spike in 0..3 {
        let x0 = spike * 11;
        for row in 0..10 {
            let w = 10 - row;
            let x = x0 + row / 2;
            let y = 31 - row;
            rect(&mut img, x, y, w.max(1), 1, if row < 2 { dark } else { body });
        }
    }
    to_tex(img)
}

pub struct TorchSprites {
    pub base: Texture2D,
    pub flame: [Texture2D; 3],
}

impl TorchSprites {
    pub fn generate() -> Self {
        let mut base = canvas(12, 24);
        let wood = Color::from_rgba(96, 64, 42, 255);
        let iron = Color::from_rgba(56, 54, 60, 255);
        rect(&mut base, 4, 8, 4, 16, wood);
        rect(&mut base, 2, 6, 8, 3, iron);

        let make_flame = |h: i32, wide: bool| {
            let mut img = canvas(12, 16);
            let outer = Color::from_rgba(230, 120, 30, 255);
            let inner = Color::from_rgba(250, 200, 60, 255);
            let w = if wide { 8 } else { 6 };
            let x0 = (12 - w) / 2;
            rect(&mut img, x0, 16 - h, w, h, outer);
            rect(&mut img, x0 + 2, 16 - h + 2, (w - 4).max(2), (h - 4).max(2), inner);
            img
        };

        Self {
            base: to_tex(base),
            flame: [
                to_tex(make_flame(11, false)),
                to_tex(make_flame(13, true)),
                to_tex(make_flame(10, true)),
            ],
        }
    }
}

pub fn generate_heart(filled: bool) -> Texture2D {
    let mut img = canvas(10, 9);
    let c = if filled {
        Color::from_rgba(220, 60, 70, 255)
    } else {
        Color::from_rgba(60, 50, 60, 255)
    };
    rect(&mut img, 1, 0, 3, 2, c);
    rect(&mut img, 6, 0, 3, 2, c);
    rect(&mut img, 0, 2, 10, 3, c);
    rect(&mut img, 1, 5, 8, 1, c);
    rect(&mut img, 2, 6, 6, 1, c);
    rect(&mut img, 3, 7, 4, 1, c);
    rect(&mut img, 4, 8, 2, 1, c);
    to_tex(img)
}

pub fn generate_gem_icon() -> Texture2D {
    generate_gem(0)
}
