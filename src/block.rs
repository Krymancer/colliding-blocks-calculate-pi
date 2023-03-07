use macroquad::prelude::*;

pub struct Block {
   pub m: f32,
   pub v: f32,
   pub rect: Rect,
}

impl Block {
   pub fn new(x: f32, w: f32, m: f32, v: f32) -> Self {
      Self { 
        rect: Rect::new(x, screen_height() - w, w, w),
        m,
        v,
      }
   }

   pub fn reverse(&mut self) {
      self.v = -self.v;
   }

   pub fn hit_wall(&self) -> bool {
      self.rect.x < 0.0
   }

    pub fn collide(&self, other: &Block) -> bool {
        self.rect.overlaps(&other.rect)
    }

    pub fn elastic_collision(&self, other: &Block) -> (f32,f32) {
        let m1 = self.m;
        let m2 = other.m;
        let u1 = self.v;
        let u2 = other.v;

        let v1 = ((m1 - m2) / (m1 + m2)) * u1 + ((2.0 * m2) / (m1 + m2)) * u2;
        let v2 = ((2.0 * m1) / (m1 + m2)) * u1 + ((m2 - m1) / (m1 + m2)) * u2;

        (v1,v2)
   }

   pub fn set_velocity(&mut self, v: f32) {
      self.v = v;
   }

   pub fn update(&mut self) {
      self.rect.x += self.v;
   }

   pub fn show(&self, color: Color) {
      draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
   }
}
