use itertools::Itertools;

pub struct Color
{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color
{
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        Self {
            r, g, b, a
        }
    }

    pub fn from_hex(code: &str, a: u8) -> Option<Self>
    {
        let r = u8::from_str_radix(code.get(0..2)?, 16).ok()?;
        let g = u8::from_str_radix(code.get(2..4)?, 16).ok()?;
        let b = u8::from_str_radix(code.get(4..6)?, 16).ok()?;

        Some(
            Self { r, g, b, a }
        )
    }

    pub fn to_floats(&self) -> (f32, f32, f32, f32)
    {
        [self.r, self.g, self.b, self.a]
            .iter()
            .map(|x| (*x as f32 / u8::MAX as f32).clamp(0.0, 1.0))
            .collect_tuple()
            .expect("")
    }
}
