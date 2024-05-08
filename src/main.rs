use ::rand::prelude::*;
use itertools::Itertools;
use macroquad::prelude::*;

/*
define a struct that will represent a body in the simulation
input : masse : f64 : the mass of the body
        color : String : the color of the body
        position : (f64, f64) : the position of the body
        velocity : (f64, f64) : the velocity of the body
output : Body : the body that was created
*/
struct Body {
    masse: f64,
    color: String,
    position: (f64, f64),
    velocity: (f64, f64),
}

#[macroquad::main("test")]
async fn main() {
    const GRAVITY: f64 = 0.0000000667;
    const STEP: f64 = 10.0;
    const NUMBER_OF_BODIES: usize = 3; // after 3, will spawn random bodies
    let mut rng = thread_rng();

    // create the default bodies
    let mut body1: Body = Body {
        masse: 10000000.0,
        color: "red".to_string(),
        position: (
            0.0 + rng.gen_range(-10.0..10.0),
            0.0 + rng.gen_range(-10.0..10.0),
        ),
        velocity: (-0.02, 0.0),
    };

    let mut body2: Body = Body {
        masse: 10000000.0,
        color: "blue".to_string(),
        position: (
            0.0 + rng.gen_range(-10.0..10.0),
            100.0 + rng.gen_range(-10.0..10.0),
        ),
        velocity: (0.0, 0.0),
    };

    let mut body3: Body = Body {
        masse: 10000000.0,
        color: "green".to_string(),
        position: (
            10.0 + rng.gen_range(-10.0..10.0),
            60.0 + rng.gen_range(-10.0..10.0),
        ),
        velocity: (0.02, 0.0),
    };

    // create additional bodies with random position and velocity
    let mut bodies: Vec<Body> = Vec::new();
    let mut rng = thread_rng();
    for i in 0..NUMBER_OF_BODIES {
        let mut mult = 1.0;
        if i % 2 == 0 {
            mult = -1.0;
        }

        // randomize the position and velocity
        let position = (
            i as f64 * rng.gen_range(-20.0..20.0) * mult,
            i as f64 * rng.gen_range(-20.0..20.0),
        );
        let velocity = (0.0, i as f64 * 0.003 * mult);
        let body = Body {
            masse: 10000000.0,
            color: "red".to_string(),
            position: position,
            velocity: velocity,
        };
        bodies.push(body);
    }
    bodies[0] = body1;
    bodies[1] = body2;
    bodies[2] = body3;

    fn calculate_force(body1: &Body, body2: &Body) -> (f64, f64, f64, f64) {
        /*
        calculate the force between two bodies
        input : body1 : &Body : the first body
                body2 : &Body : the second body
        output : (f64, f64, f64, f64) : the x and y forces of each body
        */
        let distance = ((body2.position.0 - body1.position.0).powi(2)
            + (body2.position.1 - body1.position.1).powi(2))
        .sqrt();
        if distance < 10.0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let force = GRAVITY * (body1.masse * body2.masse) / distance.powi(2);
        let b1_force_x = force / 2.0 * (body2.position.0 - body1.position.0) / distance;
        let b1_force_y = force / 2.0 * (body2.position.1 - body1.position.1) / distance;
        let b2_force_x = b1_force_x * -1.0;
        let b2_force_y = b1_force_y * -1.0;

        // calculate x and y forces of each body

        // check if the angle is 0 or 90
        if body2.position.0 - body1.position.0 == 0.0 {
            return (0.0, b1_force_y, 0.0, b2_force_y);
        }

        if body2.position.1 - body1.position.1 == 0.0 {
            return (b1_force_x, 0.0, b2_force_x, 0.0);
        }

        // body1

        (b1_force_x, b1_force_y, b2_force_x, b2_force_y)
    }
    fn update_velocity(force_x: f64, force_y: f64, body: &mut Body) {
        /*
        update the velocity of a body
        input : force_x : f64 : the x force
                force_y : f64 : the y force
                body : &mut Body : the body to update
        output : None
        */
        let acceleration_x = force_x / body.masse;
        let acceleration_y = force_y / body.masse;
        body.velocity.0 += acceleration_x * STEP;
        body.velocity.1 += acceleration_y * STEP;
    }

    // create all the combinations of bodies to calculate the forces
    let it = (0..NUMBER_OF_BODIES).combinations(2);
    let mut combinations = Vec::new();
    for i in it {
        combinations.push((i[0], i[1]));
    }

    loop {
        // update positions in the next step
        for (i, j) in combinations.clone() {
            let (b1_force_x, b1_force_y, b2_force_x, b2_force_y) =
                calculate_force(&bodies[i], &bodies[j]);

            update_velocity(b1_force_x, b1_force_y, &mut bodies[i]);
            update_velocity(b2_force_x, b2_force_y, &mut bodies[j]);
        }

        //update positions in the next step
        for body in bodies.iter_mut() {
            body.position.0 += body.velocity.0 * STEP;

            body.position.1 += body.velocity.1 * STEP;
        }
        // draw the bodies on the scren
        clear_background(WHITE);
        for body in bodies.iter() {
            let circle = Circle::new(body.position.0 as f32, body.position.1 as f32, 10.0);
            draw_circle(
                circle.x + screen_width() / 2.0,
                circle.y + screen_height() / 2.0,
                circle.r,
                match body.color.as_str() {
                    "red" => RED,
                    "blue" => BLUE,
                    "green" => GREEN,
                    _ => RED,
                },
            );
        }

        next_frame().await;
    }
}
