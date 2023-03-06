use macroquad::prelude::*;

pub struct Block {
   x: f64,
   y: f64,
   w: f64,
   h: f64,
   m: f64,
   v: f64,
}

impl Block {
   pub fn new(x: f64, y: f64, w: f64, h: f64, m: f64, v: f64) -> Self {
      Self { x, y, w, h, m, v }
   }

   pub fn reverse(&mut self) {
      self.v *= -1.0;
   }

   pub fn hit_wall(&self) -> bool {
      self.x < 0.0
   }

   pub fn collide(&self, other: &Block) -> bool {
      !(self.x + self.w < other.x || self.x > other.x + other.w)
   }

   pub fn bounce(&self, other: &Block) -> f64 {
      ((self.m - other.m) / (self.m + other.m) * self.v)
         + ((2.0 * other.m) / (self.m + other.m) * other.v)
   }

   pub fn set_velocity(&mut self, v: f64) {
      self.v = v;
   }

   pub fn update(&mut self) {
      self.x += self.v;
   }

   pub fn show(&self, color: Color) {
      draw_rectangle(self.x as f32, self.y as f32, self.w as f32, self.h as f32, color);
   }
}
