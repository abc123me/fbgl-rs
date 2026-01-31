#[cfg(feature = "fbdev")]
pub mod framebuffer;

#[cfg(feature = "text")]
pub mod text;

#[cfg(feature = "img")]
pub mod image;

#[derive(Clone, Copy)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait ReprColor: Sized + Copy {
    fn from_rgb(r: u8, g: u8, b: u8) -> Self;
    fn to_rgb(&self) -> [u8; 3];

    fn new(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl ReprColor for RGBA {
    fn from_rgb(_r: u8, _g: u8, _b: u8) -> Self {
        RGBA {
            r: _r,
            g: _g,
            b: _b,
            a: 255,
        }
    }
    fn to_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

pub trait GraphicsOperations: GraphicsRenderer {
    /* For branch prediction reasons, all of these are assumed to
     * have appropriate input values, and if not you're fucked */
    fn clear(&mut self, col: Self::Color);
    fn rect(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16);
    fn rect_outline(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16);
    fn circle(&mut self, col: Self::Color, x: u16, y: u16, s: u16);
    fn circle_outline(&mut self, col: Self::Color, x: u16, y: u16, s: u16);
    fn ellipse(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16);
    fn ellipse_outline(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16);
    fn line(&mut self, col: Self::Color, x: u16, y: u16, x2: u16, y2: u16);
    fn hline_bounded(&mut self, col: Self::Color, x: u16, y1: u16, y2: u16);
    fn vline_bounded(&mut self, col: Self::Color, y: u16, x1: u16, x2: u16);
    fn hline(&mut self, col: Self::Color, x: u16);
    fn vline(&mut self, col: Self::Color, y: u16);
}

pub trait GraphicsRenderer {
    type Color: ReprColor;

    fn get_pixel(&self, x: u16, y: u16) -> Self::Color;
    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;
    fn get_size(&self) -> (u16, u16) {
        (self.get_width(), self.get_height())
    }
    fn get_num_pixels(&self) -> u32 {
        (self.get_width() as u32) * (self.get_height() as u32)
    }

    fn set_pixel(&mut self, col: Self::Color, x: u16, y: u16);
    fn set_pixels(&mut self, pixels: &Vec<Self::Color>) {
        let w = self.get_width();
        let h = self.get_height();
        assert!(
            pixels.len() == self.get_num_pixels() as usize,
            "Given incorrect amount of pixels, given {}, got {}!",
            pixels.len(),
            self.get_num_pixels() as usize,
        );
        for y in 0..h {
            let p = y * w;
            for x in 0..w {
                self.set_pixel(pixels[(x + p) as usize], x, y);
            }
        }
    }
    fn blend_pixel(&mut self, col: Self::Color, x: u16, y: u16, alpha: u8) {
        let (old, new) = (self.get_pixel(x as u16, y as u16).to_rgb(), col.to_rgb());
        let alpha16 = alpha as u16;
        let alpha_orig = 256_u16 - alpha16;
        let r = ((old[0] as u16 * alpha_orig) + (new[0] as u16 * alpha16)) / 256;
        let g = ((old[1] as u16 * alpha_orig) + (new[1] as u16 * alpha16)) / 256;
        let b = ((old[2] as u16 * alpha_orig) + (new[2] as u16 * alpha16)) / 256;
        self.set_pixel(Self::Color::from_rgb(r as u8, g as u8, b as u8), x, y);
    }
}

impl<T: GraphicsRenderer> GraphicsOperations for T {
    fn rect(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16) {
        for xi in 0..w {
            for yi in 0..h {
                self.set_pixel(col, x + xi, y + yi);
            }
        }
    }
    fn rect_outline(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16) {
        self.vline_bounded(col, x, y, y + h);
        self.vline_bounded(col, x + w, y, y + h);
        self.hline_bounded(col, y, x, x + w);
        self.hline_bounded(col, y + h, x, x + w);
        self.set_pixel(col, x + w, y + h);
    }
    fn circle(&mut self, col: Self::Color, x: u16, y: u16, s: u16) {
        self.ellipse(col, x, y, s, s)
    }
    fn circle_outline(&mut self, col: Self::Color, x: u16, y: u16, s: u16) {
        self.ellipse_outline(col, x, y, s, s)
    }
    fn ellipse(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16) {
        let wh = w / 2;
        let hh = h / 2;
        let dw2 = wh * wh;
        let dh2 = hh * hh;
        for xp in 0..wh {
            let xp2 = xp * xp;
            for yp in 0..hh {
                if xp2 + yp * yp < dh2 {
                    self.set_pixel(col, x - xp, y - yp);
                    self.set_pixel(col, x + xp, y - yp);
                    self.set_pixel(col, x - xp, y + yp);
                    self.set_pixel(col, x + xp, y + yp);
                }
            }
        }
    }
    fn ellipse_outline(&mut self, col: Self::Color, x: u16, y: u16, w: u16, h: u16) {
        //todo
    }
    fn line(&mut self, col: Self::Color, x1: u16, y1: u16, x2: u16, y2: u16) {
        let dx = (x2 as i32) - (x1 as i32);
        let dy = (y2 as i32) - (y1 as i32);
        // Bitwise OR here is faster then max
        // and has no branch prediction impacts
        let ds = dx.abs() | dy.abs();
        for i in 0..ds {
            let xp = x1 as i32 + (dx * i) / ds;
            let yp = y1 as i32 + (dy * i) / ds;
            self.set_pixel(col, xp as u16, yp as u16);
        }
    }
    fn vline_bounded(&mut self, col: Self::Color, xp: u16, y1: u16, y2: u16) {
        assert!(y2 > y1, "y2 must be greater then y1 for vline_bounded!");
        for yp in y1..y2 {
            self.set_pixel(col, xp, yp);
        }
    }
    fn hline_bounded(&mut self, col: Self::Color, yp: u16, x1: u16, x2: u16) {
        assert!(x2 > x1, "x2 must be greater then x1 for vline_bounded!");
        for xp in x1..x2 {
            self.set_pixel(col, xp, yp);
        }
    }
    fn vline(&mut self, col: Self::Color, p: u16) {
        self.vline_bounded(col, p, 0, self.get_height());
    }
    fn hline(&mut self, col: Self::Color, p: u16) {
        self.hline_bounded(col, p, 0, self.get_width());
    }
    fn clear(&mut self, col: Self::Color) {
        /* chose to iterate over y here since most displays
         * draw on a row-by-row / scan line basis */
        for p in 0..self.get_height() {
            self.hline(col, p);
        }
    }
}

pub struct BufferedRenderer<T: GraphicsRenderer> {
    base: T,
    buffer_width: u16,
    buffer_height: u16,
    buffer: Vec<T::Color>,
}

impl<T: GraphicsRenderer> BufferedRenderer<T> {
    pub fn new(mut base_renderer: T) -> Self {
        let mut out = BufferedRenderer::<T> {
            base: base_renderer,
            buffer_width: 0,
            buffer_height: 0, /* these are set by resize_to_base */
            buffer: Vec::new(),
        };
        out.resize_to_base();
        out
    }

    pub fn resize_to_base(&mut self) {
        self.buffer_width = self.base.get_width();
        self.buffer_height = self.base.get_height();
        self.buffer = Vec::<T::Color>::with_capacity(self.base.get_num_pixels() as usize);
        for i in 0..self.base.get_num_pixels() {
            self.buffer.push(T::Color::new(0, 0, 0));
        }
        println!(
            "BufferedRenderer size set to {} x {} ({} pixels)",
            self.buffer_width,
            self.buffer_height,
            self.base.get_num_pixels()
        );
    }

    pub fn push_buffer(&mut self) {
        self.base.set_pixels(&self.buffer)
    }
}

impl<T: GraphicsRenderer> GraphicsRenderer for BufferedRenderer<T> {
    type Color = T::Color;

    fn get_pixel(&self, x: u16, y: u16) -> T::Color {
        let w = self.get_width();
        self.buffer[(x + y * w) as usize]
    }
    fn get_width(&self) -> u16 {
        self.buffer_width
    }
    fn get_height(&self) -> u16 {
        self.buffer_height
    }

    fn set_pixel(&mut self, col: T::Color, x: u16, y: u16) {
        let w = self.get_width();
        self.buffer[(x + y * w) as usize] = col;
    }
}
