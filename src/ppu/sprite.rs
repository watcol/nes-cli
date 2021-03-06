use super::{RegisterIO, Registers};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectAttributeMemory(pub [Sprite; 64]);

impl Default for ObjectAttributeMemory {
    fn default() -> Self {
        Self([Sprite::default(); 64])
    }
}

impl ObjectAttributeMemory {
    pub fn read(&self, addr: u8) -> u8 {
        self.0[(addr / 4) as usize].read(addr % 4)
    }

    pub fn write(&mut self, addr: u8, value: u8) {
        self.0[(addr / 4) as usize].write(addr % 4, value)
    }

    pub fn sync_registers(&mut self, reg: &mut Registers) {
        match reg.io {
            RegisterIO::WriteOamAddr => {
                reg.oam_data = self.read(reg.oam_addr);
            }
            RegisterIO::WriteOamData => {
                self.write(reg.oam_addr, reg.oam_data);
                reg.oam_addr += 1;
                reg.oam_data = self.read(reg.oam_addr);
            }
            _ => {}
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Sprite {
    pub y: u8,
    pub tile: u8,
    pub attr: SpriteAttribute,
    pub x: u8,
}

impl Sprite {
    pub fn read(&self, addr: u8) -> u8 {
        match addr {
            0 => self.y,
            1 => self.tile,
            2 => self.attr.as_u8(),
            3 => self.x,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u8, value: u8) {
        match addr {
            0 => self.y = value,
            1 => self.tile = value,
            2 => self.attr = SpriteAttribute::from_u8(value),
            3 => self.x = value,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SpriteAttribute {
    pub vertical_flip: bool,
    pub horizontal_flip: bool,
    pub hide: bool,
    pub palette: u8,
}

impl Default for SpriteAttribute {
    fn default() -> Self {
        Self {
            vertical_flip: false,
            horizontal_flip: false,
            hide: true,
            palette: 0,
        }
    }
}

impl SpriteAttribute {
    fn from_u8(bits: u8) -> Self {
        Self {
            vertical_flip: bits & 0x80 == 0x80,
            horizontal_flip: bits & 0x40 == 0x40,
            hide: bits & 0x20 == 0x20,
            palette: bits & 0x3,
        }
    }

    fn as_u8(&self) -> u8 {
        (self.vertical_flip as u8) * 0x80
            + (self.horizontal_flip as u8) * 0x40
            + (self.hide as u8) * 0x20
            + self.palette
    }
}
