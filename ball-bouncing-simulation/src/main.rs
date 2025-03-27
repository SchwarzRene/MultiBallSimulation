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

    let mut balls = Vec::new();

    for i in  0..10 {
        let x_pos = i as f64 * 20.;
        
        let mut x_direction = 1.;
        if i % 1 == 0{ x_direction = -1.  }

        let x_vel = i as f64 * 5. * x_direction;

        let position: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ x_pos, 400. ] );
        let velocity: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ x_vel, 0. ] );
        let acceleration: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>> = Array1::from( vec![ 0., 9.8 ] );

        let c : RetschisColour;
        match i % 3 {
            0 => c = RetschisColour::Red,
            1 => c = RetschisColour::Blue,
            _ => c = RetschisColour::Green
        }

        let s : Size;
        match i % 3{
            0 => s = Size::S,
            1 => s = Size::M,
            _ => s = Size::L
        }

        let b = Ball{
            pos : position,
            v : velocity,
            a : acceleration,
            color : c,
            size : s
        };

        balls.push( b )
    }
    

    loop {
        clear_background( BLACK );

        let width = screen_width();
        let height = screen_height();

        for ball in balls.iter_mut(){
            draw_circle( ball.pos[0] as f32, height - ball.pos[1] as f32, size_convertion( &ball.size ), get_color( &ball.color ) );

            update_ball( ball, 0.2 );
            detect_collision_with_walls( ball, width, height, 1. );
        }

        detect_ball_collision( &mut balls );
        
        next_frame().await;
    };
}



fn update_ball( b : &mut Ball, time_constant : f64 ){
    b.pos = &b.pos - &(&b.v*time_constant/2.);
    b.v = &b.v + &(&b.a*time_constant);
}

fn detect_collision_with_walls( b : &mut Ball, width : f32, height : f32, dumping_factor : f64 ){
    let lower_edge = &b.pos[ 1 ] - size_convertion( &b.size ) as f64;
    if lower_edge <= 0.{
        b.v[ 1 ] = -&b.v[ 1 ] * dumping_factor;
        b.pos[ 1 ] = 0. + size_convertion( &b.size ) as f64;
    }

    let upper_edge = &b.pos[ 1 ] + size_convertion( &b.size ) as f64;
    if upper_edge >= height as f64{
        b.v[ 1 ] = -&b.v[ 1 ] * dumping_factor;
        b.pos[ 1 ] = (height - size_convertion( &b.size )) as f64;
    }

    let left_edge = &b.pos[ 0 ] - size_convertion( &b.size ) as f64;
    if left_edge <= 0. {
        b.v[ 0 ] = -&b.v[ 0 ] * dumping_factor;
        b.pos[ 0 ] = 0. + size_convertion( &b.size ) as f64;
    }

    let right_edge = &b.pos[ 0 ] + size_convertion( &b.size ) as f64;
    if right_edge >= width as f64 {
        b.v[ 0 ] = -&b.v[ 0 ] * dumping_factor;
        b.pos[ 0 ] = (width - size_convertion( &b.size )) as f64;
    }
}

fn detect_ball_collision( balls : &mut Vec<Ball> ){
    let mut distance_array = vec![ vec![ 0.; balls.len() ]; balls.len() ];

    for ( idx1, b1 ) in balls.iter().enumerate(){
        for ( idx2, b2 ) in balls.iter().enumerate(){
            let distance = (&b1.pos - &b2.pos).iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
            distance_array[ idx1 ][ idx2 ] = distance;
        }
    }

    let mut changed_balls = vec![ vec![ ] ];
    for idx1 in 0..balls.len(){
        for idx2 in 0..balls.len(){
            if idx1 != idx2 {
                let distance = distance_array[ idx1 ][ idx2 ] as f32;
                if distance <= size_convertion( &balls[ idx1 ].size ) + size_convertion( &balls[ idx2 ].size ){
                    
                    let mut already_resolved = false;
                    for idx_pair in changed_balls.iter(){
                        if idx_pair == &vec![ idx1, idx2 ] || idx_pair == &vec![ idx2, idx1 ]{
                            already_resolved = true;
                        }
                    }

                    if !already_resolved{
                        let v1 = balls[idx1].v.clone();
                        let v2 = balls[idx2].v.clone();

                        balls[idx1].v = v2;
                        balls[idx2].v = v1;

                        changed_balls.push( vec![ idx1, idx2 ] );
                    }
                }
            }
        }
    }
}

fn size_convertion( size : &Size ) -> f32{
    match size{
        Size::S => 5.,
        Size::M => 10.,
        Size::L => 20.
    }
}

fn get_color( c : &RetschisColour ) -> Color{
    match c{
        RetschisColour::Red => RED,
        RetschisColour::Green => GREEN,
        RetschisColour::Blue => BLUE
    }
}