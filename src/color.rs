pub const WHITE: Rgb = Rgb(255u8, 255u8, 255u8);
pub const BLACK: Rgb = Rgb(0u8, 0u8, 0u8);
pub const FIRE: [Rgb; 31] = [
    Rgb(0, 0, 0),
    Rgb(30, 0, 0),
    Rgb(60, 0, 0),
    Rgb(90, 10, 0),
    Rgb(120, 20, 0),
    Rgb(150, 40, 0),
    Rgb(180, 60, 0),
    Rgb(210, 80, 0),
    Rgb(240, 100, 0),
    Rgb(255, 120, 0),
    Rgb(255, 140, 10),
    Rgb(255, 160, 20),
    Rgb(255, 180, 40),
    Rgb(255, 200, 60),
    Rgb(255, 220, 100),
    Rgb(255, 240, 150),
    Rgb(255, 255, 200),
    Rgb(240, 240, 255),
    Rgb(200, 220, 255),
    Rgb(150, 190, 255),
    Rgb(100, 150, 255),
    Rgb(60, 120, 240),
    Rgb(30, 90, 220),
    Rgb(10, 60, 180),
    Rgb(0, 40, 140),
    Rgb(0, 25, 100),
    Rgb(0, 15, 70),
    Rgb(0, 10, 40),
    Rgb(0, 5, 20),
    Rgb(0, 2, 10),
    Rgb(0, 0, 0),
];
pub const XAOS: [Rgb; 31] = [
    Rgb(0, 0, 0),
    Rgb(120, 119, 238),
    Rgb(24, 7, 25),
    Rgb(197, 66, 28),
    Rgb(29, 18, 11),
    Rgb(135, 46, 71),
    Rgb(24, 27, 13),
    Rgb(241, 230, 128),
    Rgb(17, 31, 24),
    Rgb(240, 162, 139),
    Rgb(11, 4, 30),
    Rgb(106, 87, 189),
    Rgb(29, 21, 14),
    Rgb(12, 140, 118),
    Rgb(10, 6, 29),
    Rgb(50, 144, 77),
    Rgb(22, 0, 24),
    Rgb(148, 188, 243),
    Rgb(4, 32, 7),
    Rgb(231, 146, 14),
    Rgb(10, 13, 20),
    Rgb(184, 147, 68),
    Rgb(13, 28, 3),
    Rgb(169, 248, 152),
    Rgb(4, 0, 34),
    Rgb(62, 83, 48),
    Rgb(7, 21, 22),
    Rgb(152, 97, 184),
    Rgb(8, 3, 12),
    Rgb(247, 92, 235),
    Rgb(31, 32, 16),
];

#[derive(Clone, Copy, Debug)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(red, green, blue)
    }

    pub fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
        Rgb(
            ((((b.0 as f32 / 255. - a.0 as f32 / 255.) * t)
                + a.0 as f32 / 255.)
                * 255.)
                .round() as u8,
            ((((b.1 as f32 / 255. - a.1 as f32 / 255.) * t)
                + a.1 as f32 / 255.)
                * 255.)
                .round() as u8,
            ((((b.2 as f32 / 255. - a.2 as f32 / 255.) * t)
                + a.2 as f32 / 255.)
                * 255.)
                .round() as u8,
        )
    }
}

impl Into<u32> for Rgb {
    fn into(self) -> u32 {
        self.2 as u32 | (self.1 as u32) << 8 | (self.0 as u32) << 16
    }
}

pub fn stretch(steps: u32, palette: &[Rgb; 31]) -> Vec<Rgb> {
    let mut stretched = vec![BLACK; palette.len() * steps as usize];
    for i in 0..palette.len() * steps as usize {
        let index = (i as f32 / steps as f32) as usize;
        let a = palette[index];
        let b = palette[(index + 1) % palette.len()];
        let step = (i % steps as usize) as f32 / steps as f32;
        stretched[i] = Rgb::lerp(a, b, step);
    }

    stretched
}
