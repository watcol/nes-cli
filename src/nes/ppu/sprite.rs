use image::Rgb;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sprite([[u8; 8]; 8]);

impl Sprite {
    pub fn new<T: AsRef<[u8]>>(chr_rom: T, index: u8) -> Self {
        let chunk = chr_rom.as_ref().chunks(0x10).nth(index as usize).unwrap();
        let mut sprite = [[0; 8]; 8];
        for i in 0..8 {
            let mut byte1 = chunk[i];
            let mut byte2 = chunk[i + 8];
            for j in (0..8).rev() {
                let bit1 = byte1 % 2;
                let bit2 = byte2 % 2;
                byte1 /= 2;
                byte2 /= 2;
                sprite[i][j] = bit2 * 2 + bit1;
            }
        }

        Self(sprite)
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        self.0[y as usize][x as usize]
    }
}

static COLORS: [Rgb<u8>; 64] = [
    Rgb([0x80, 0x80, 0x80]),
    Rgb([0x00, 0x3D, 0xA6]),
    Rgb([0x00, 0x12, 0xB0]),
    Rgb([0x44, 0x00, 0x96]),
    Rgb([0xA1, 0x00, 0x5E]),
    Rgb([0xC7, 0x00, 0x28]),
    Rgb([0xBA, 0x06, 0x00]),
    Rgb([0x8C, 0x17, 0x00]),
    Rgb([0x5C, 0x2F, 0x00]),
    Rgb([0x10, 0x45, 0x00]),
    Rgb([0x05, 0x4A, 0x00]),
    Rgb([0x00, 0x47, 0x2E]),
    Rgb([0x00, 0x41, 0x66]),
    Rgb([0x00, 0x00, 0x00]),
    Rgb([0x05, 0x05, 0x05]),
    Rgb([0x05, 0x05, 0x05]),
    Rgb([0xC7, 0xC7, 0xC7]),
    Rgb([0x00, 0x77, 0xFF]),
    Rgb([0x21, 0x55, 0xFF]),
    Rgb([0x82, 0x37, 0xFA]),
    Rgb([0xEB, 0x2F, 0xB5]),
    Rgb([0xFF, 0x29, 0x50]),
    Rgb([0xFF, 0x22, 0x00]),
    Rgb([0xD6, 0x32, 0x00]),
    Rgb([0xC4, 0x62, 0x00]),
    Rgb([0x35, 0x80, 0x00]),
    Rgb([0x05, 0x8F, 0x00]),
    Rgb([0x00, 0x8A, 0x55]),
    Rgb([0x00, 0x99, 0xCC]),
    Rgb([0x21, 0x21, 0x21]),
    Rgb([0x09, 0x09, 0x09]),
    Rgb([0x09, 0x09, 0x09]),
    Rgb([0xFF, 0xFF, 0xFF]),
    Rgb([0x0F, 0xD7, 0xFF]),
    Rgb([0x69, 0xA2, 0xFF]),
    Rgb([0xD4, 0x80, 0xFF]),
    Rgb([0xFF, 0x45, 0xF3]),
    Rgb([0xFF, 0x61, 0x8B]),
    Rgb([0xFF, 0x88, 0x33]),
    Rgb([0xFF, 0x9C, 0x12]),
    Rgb([0xFA, 0xBC, 0x20]),
    Rgb([0x9F, 0xE3, 0x0E]),
    Rgb([0x2B, 0xF0, 0x35]),
    Rgb([0x0C, 0xF0, 0xA4]),
    Rgb([0x05, 0xFB, 0xFF]),
    Rgb([0x5E, 0x5E, 0x5E]),
    Rgb([0x0D, 0x0D, 0x0D]),
    Rgb([0x0D, 0x0D, 0x0D]),
    Rgb([0xFF, 0xFF, 0xFF]),
    Rgb([0xA6, 0xFC, 0xFF]),
    Rgb([0xB3, 0xEC, 0xFF]),
    Rgb([0xDA, 0xAB, 0xEB]),
    Rgb([0xFF, 0xA8, 0xF9]),
    Rgb([0xFF, 0xAB, 0xB3]),
    Rgb([0xFF, 0xD2, 0xB0]),
    Rgb([0xFF, 0xEF, 0xA6]),
    Rgb([0xFF, 0xF7, 0x9C]),
    Rgb([0xD7, 0xE8, 0x95]),
    Rgb([0xA6, 0xED, 0xAF]),
    Rgb([0xA2, 0xF2, 0xDA]),
    Rgb([0x99, 0xFF, 0xFC]),
    Rgb([0xDD, 0xDD, 0xDD]),
    Rgb([0x11, 0x11, 0x11]),
    Rgb([0x11, 0x11, 0x11]),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColoredSprite([[Rgb<u8>; 8]; 8]);

impl ColoredSprite {
    pub fn new(sprite: &Sprite, pallete: &[u8; 4]) -> Self {
        let mut res = [[Rgb([0, 0, 0]); 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                res[y as usize][x as usize] = COLORS[pallete[sprite.get(x, y) as usize] as usize];
            }
        }
        Self(res)
    }

    pub fn get(&self, x: u8, y: u8) -> Rgb<u8> {
        self.0[y as usize][x as usize]
    }
}
