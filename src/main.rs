mod block;

use std::env;
use macroquad::prelude::*;

use macroquad::time::get_fps;
use block::Block;

fn window_conf() -> Conf {
    Conf {
        window_title: "Computing PI".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // in case kernel panics, show stack trace
    let mut collisions = 0;
    let m : f32 = 1e10;
    let v : f32 = -1.0;// -5e6;
    let dt = 1e4;
    
    let mut block1 = Block::new(100.0, 100.0, 1.0, 0.0);
    let mut block2 = Block::new(300.0, 201.0, m, v/dt);

    loop {

        for _ in 0..(dt as u32) {
        if block1.collide(&block2) { 
            let (v1,v2) = block1.elastic_collision(&block2);
            block1.set_velocity(v1);
            block2.set_velocity(v2);
            collisions += 1;
        }

        if block1.hit_wall() {
            block1.reverse();
            collisions += 1;
        }
        

        block1.update();
        block2.update();

        }

        clear_background(BLACK);

        let text = format!("Collisions: {}", collisions);
        draw_text(text.as_str(), 20.0, 20.0, 30.0, WHITE);
 
        let fps_text = format!("FPS: {}", get_fps());
        draw_text(fps_text.as_str(), screen_width() - (15 * fps_text.len()) as f32, 20.0, 30.0, WHITE);

        block1.show(BLUE);
        block2.show(RED); 

        next_frame().await
    }
}
