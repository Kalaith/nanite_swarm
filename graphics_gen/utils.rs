use image::{Rgba, RgbaImage};

pub const TILE_WIDTH: u32 = 32;
pub const TILE_HEIGHT: u32 = 32;

pub fn create_tile_base(color: Rgba<u8>) -> RgbaImage {
    let mut img = RgbaImage::new(TILE_WIDTH, TILE_HEIGHT);
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            img.put_pixel(x, y, color);
        }
    }
    img
}

pub fn add_noise(img: &mut RgbaImage, amount: i16, seed: u32) {
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let n = hashed_noise(x, y, seed) as i16;
            let delta = (n % (amount * 2 + 1)) - amount;
            let p = *img.get_pixel(x, y);
            img.put_pixel(
                x,
                y,
                Rgba([
                    clamp_u8(p[0] as i16 + delta),
                    clamp_u8(p[1] as i16 + delta),
                    clamp_u8(p[2] as i16 + delta),
                    p[3],
                ]),
            );
        }
    }
}

pub fn add_edge_darkening(img: &mut RgbaImage, depth: u32, amount: i16) {
    for y in 0..TILE_HEIGHT {
        for x in 0..TILE_WIDTH {
            let edge_dist = x.min(y).min(TILE_WIDTH - 1 - x).min(TILE_HEIGHT - 1 - y);
            if edge_dist < depth {
                let scale = (depth - edge_dist) as i16;
                let delta = -amount * scale as i16 / depth as i16;
                let p = *img.get_pixel(x, y);
                img.put_pixel(
                    x,
                    y,
                    Rgba([
                        clamp_u8(p[0] as i16 + delta),
                        clamp_u8(p[1] as i16 + delta),
                        clamp_u8(p[2] as i16 + delta),
                        p[3],
                    ]),
                );
            }
        }
    }
}

pub fn draw_rect(img: &mut RgbaImage, x: u32, y: u32, w: u32, h: u32, color: Rgba<u8>) {
    for yy in y..(y + h).min(TILE_HEIGHT) {
        for xx in x..(x + w).min(TILE_WIDTH) {
            img.put_pixel(xx, yy, color);
        }
    }
}

pub fn draw_circle(img: &mut RgbaImage, cx: i32, cy: i32, r: i32, color: Rgba<u8>) {
    let r2 = r * r;
    for y in (cy - r)..=(cy + r) {
        for x in (cx - r)..=(cx + r) {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= r2 {
                if x >= 0 && y >= 0 && (x as u32) < TILE_WIDTH && (y as u32) < TILE_HEIGHT {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

pub fn draw_line(img: &mut RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgba<u8>) {
    let mut x = x0;
    let mut y = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x >= 0 && y >= 0 && (x as u32) < TILE_WIDTH && (y as u32) < TILE_HEIGHT {
            img.put_pixel(x as u32, y as u32, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn hashed_noise(x: u32, y: u32, seed: u32) -> u32 {
    let mut v = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263));
    v ^= seed.wrapping_mul(2246822519);
    v ^= v >> 13;
    v = v.wrapping_mul(1274126177);
    v ^= v >> 16;
    v
}

fn clamp_u8(value: i16) -> u8 {
    value.clamp(0, 255) as u8
}
