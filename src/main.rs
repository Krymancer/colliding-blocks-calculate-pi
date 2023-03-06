mod block;

use std::env;
use macroquad::prelude::*;
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
    let mut collisions : u64 = 0;
    let m : f64 = 10000.0;
    let v : f64 = -1.0;// -5e6;
    
    let mut block1 = Block::new(100.0, 400.0, 100.0, 100.0, 1.0, 0.0);
    let mut block2 = Block::new(300.0, 300.0, 200.0, 200.0, m, v);


    loop {

        if block1.hit_wall() {
            block1.reverse();
            collisions += 1;
        }

        if block1.collide(&block2) {

            let v1 = block1.bounce(&block2);
            let v2 = block2.bounce(&block1);
            block1.set_velocity(v1);
            block2.set_velocity(v2);
            collisions += 1;
        }


        block1.update();
        block2.update();

        clear_background(BLACK);

        let text = format!("Collisions: {}", collisions);
        draw_text(text.as_str(), 20.0, 20.0, 30.0, WHITE);

        block1.show(BLUE);
        block2.show(RED); 

        next_frame().await
    }
}
