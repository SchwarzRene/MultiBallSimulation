use macroquad::prelude::*;
use ndarray::Array1;

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
    pos : ndarray::Array1<f64>,
    v : ndarray::Array1<f64>,
    a : ndarray::Array1<f64>,
    color : RetschisColour,
    size : Size
}

#[macroquad::main("Ball Simulation")]
async fn main() {
    let position: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ 0., 0. ] );
    let velocity: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ 0., 0. ] );
    let acceleration: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ 0., 9.8 ] );

    let b = &mut Ball{
        pos : position,
        v : velocity,
        a : acceleration,
        color : RetschisColour::Red,
        size : Size::S
    };

    loop {
        clear_background( BLACK );

        draw_circle( b.pos[0] as f32, b.pos[1] as f32, size_convertion( &b.size ), get_color( &b.color ) );

        update_ball( b, 0.1 );

        next_frame().await;
    };
}

fn update_ball( b : &mut Ball, time_constant : f64 ){
    b.pos = &b.pos - &(&b.v*time_constant);
    b.v = &b.v + &(&b.a*time_constant);
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