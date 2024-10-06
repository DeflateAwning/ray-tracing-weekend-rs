pub struct Color {
    r: f32,
    g: f32,
    b: f32
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r, g, b
        }
    }

    pub fn as_ppm_str(self: Color) -> String {
        let r_int = (self.r * 255.999) as u8;
        let g_int = (self.g * 255.999) as u8;
        let b_int: u8 = (self.b * 255.999) as u8;
        format!("{r_int} {g_int} {b_int}\n")
    }
}

