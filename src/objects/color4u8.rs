pub struct Color4u8 {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

fn remap(val: f32, oldMin: f32, oldMax: f32, newMin: f32, newMax: f32) -> f32 {
    if oldMin != oldMax && newMin != newMax {
        (((val - oldMin) * (newMax - newMin)) / (oldMax - oldMin)) + newMin
    } else {
        (newMax + newMin) / 2.0
    }
}

fn f32_to_u8(v: f32) -> u8 {
    //println!("From : [{}] to [{}]", v, res);
    ((v * 255.0) as u32 >> 16) as u8
}

impl Color4u8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            a, r, g, b
        }
    }

    pub fn from_f32s(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            a: f32_to_u8(a),
            r: f32_to_u8(r),
            g: f32_to_u8(g),
            b: f32_to_u8(b)
        }
    }

    pub fn as_u32(&self) -> u32 {
        // (self.a as u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | (self.a as u32)
        (self.b as u32) << 16 | (self.g as u32) << 8 | (self.r as u32)
    }
}