use macroquad::prelude::*;

enum RetschisColour{
    Red,
    Green,
    Blue
}

enum Size{
    S,
    M,
    L
}


struct Ball{
    pos : Vec<f64>,
    v : Vec<f64>,
    a : Vec<f64>,
    color : RetschisColour,
    size : Size
}

#[macroquad::main("Ball Simulation")]
async fn main() {
    let position = vec![ 0., 0. ];
    let velocity = vec![ 0., 0. ];
    let acceleration = vec![ 0., 0. ];

    let mut b = Ball{
        pos : position,
        v : velocity,
        a : acceleration,
        color : RetschisColour::Red,
        size : Size::S
    };

    loop {
        clear_background( BLACK );

        draw_circle( 0., 0., size_convertion( &b.size ), get_color( &b.color ) );

        update_ball( b );

        next_frame().await;
    };
}

fn update_ball( b : &mut Ball, time_constant : f64 ){
    b.pos = b.pos - b.v;
    b.v = b.v + b.a;
}

fn size_convertion( size : &Size ) -> f32{
    match size{
        Size::S => 100.,
        Size::M => 200.,
        Size::L => 300.
    }
}

fn get_color( c : &RetschisColour ) -> Color{
    match c{
        RetschisColour::Red => RED,
        RetschisColour::Green => GREEN,
        RetschisColour::Blue => BLUE
    }
}