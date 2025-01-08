use tiny_skia::*;
use crate::robot::Robot;

pub struct Painter {
  pub width: u32,
  pub height: u32,
}

impl Painter {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height
    }
  }

  pub fn draw(&mut self, filename: &str, robots: &Vec<Robot>) {
    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 127, 0, 200);
    paint.anti_alias = true;

    let mut pixmap = Pixmap::new(self.width, self.height).unwrap();
    let stroke = Stroke::default();
    
    for bot in robots {
      let bot_x = bot.get_x_as_f32() * 2.0;
      let bot_y = bot.get_y_as_f32() * 2.0;
      let path = PathBuilder::from_rect(Rect::from_ltrb(bot_x, bot_y, bot_x + 1.0, bot_y + 1.0).unwrap());
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
    
    pixmap.save_png(filename).unwrap();
  }
}