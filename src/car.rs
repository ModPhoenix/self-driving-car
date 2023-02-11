use web_sys::CanvasRenderingContext2d;

pub struct Car {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
}

impl Car {
    pub fn new(x: f64, y: f64, width: f64, height: f64, color: String) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // ctx.set_fill_style(&self.color.clone().into());
        ctx.begin_path();
        ctx.rect(
            self.x - self.width / 2.0,
            self.y - self.height / 2.0,
            self.width,
            self.height,
        );
        ctx.fill();
    }
}
