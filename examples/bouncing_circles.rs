extern crate sfml;
extern crate box2d;
extern crate time;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color, PrimitiveType, Vertex};

use box2d::world::World;
use box2d::body::BodyDef;
use box2d::body::BodyType;
use box2d::math::Vec2;

use time::Duration;
use time::get_time;

fn main() {
    let step = 1.0 / 60.0;
    let mut current_time: f64 = 0.0;
    let mut accumulator: f64 = 0.0;

    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML Example",
                                       Close,
                                       &ContextSettings::default())
                         .expect("Cannot create a new Render Window.");

    let mut world = setup_box2d();

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        let time_data = get_time();
        let new_time = (Duration::seconds(time_data.sec as i64) + Duration::nanoseconds(time_data.nsec as i64)).num_milliseconds() as f64 / 1000.0;
        let frame_time = (new_time - current_time).min(0.2);
        current_time = new_time;

        accumulator = accumulator + frame_time;
        while accumulator >= step {
            world.step(step as f32);
            accumulator -= step;
        }

        // Clear the window
        window.clear(&Color::new_rgb(0, 200, 200));
        for i in 0..world.bodies.len() {
            let ref shape = world.bodies[i].shape;
            match *shape {
                box2d::shape::shape::Shape::CircleShape{center, radius} => {
                    let mut circle = CircleShape::new().expect("Error, cannot create ball.");
                    let position = world.bodies[i].position + center;
                    circle.set_radius(radius-1.0);
                    circle.set_outline_thickness(1.0);
                    circle.set_outline_color(&Color{red: 255, green: 0, blue: 0, alpha: 255});
                    circle.set_fill_color(&Color::transparent());
                    circle.set_position(&Vector2f::new(position.x, position.y));
                    circle.set_origin(&Vector2f::new(radius, radius));
                    window.draw(&circle);
                },
                box2d::shape::shape::Shape::LineShape{point1, point2} => {
                    let point1_global = world.bodies[i].position + point1;
                    let point2_global = world.bodies[i].position + point2;
                    let points = [Vertex::new_with_pos_color(&Vector2f::new(point1_global.x, point1_global.y), &Color::blue()),
                    Vertex::new_with_pos_color(&Vector2f::new(point2_global.x, point2_global.y), &Color::blue())];
                    window.draw_primitives(&points, PrimitiveType::Lines);
                },
                box2d::shape::shape::Shape::ChainLineShape{ref points} => {
                    let mut global_points: Vec<Vertex> = Vec::new();
                    for p in points.iter() {
                        let global_point = world.bodies[i].position + *p;
                        global_points.push(Vertex::new_with_pos_color(&Vector2f::new(global_point.x, global_point.y), &Color::blue()));
                    }
                    window.draw_primitives(&global_points, PrimitiveType::LinesStrip);
                },
                _ => {
                    //Error
                }
            }
        }
        window.display();
    }
}

fn setup_box2d() -> World {
    let mut world = World::new(Vec2::new(0.0, 2.0));


    /*let circle_shape = box2d::shape::shape::Shape::CircleShape { center: Vec2::new(0.0, 0.0), radius: 20.0 };
    let circle_body_def = BodyDef{shape: circle_shape, body_type: BodyType::DynamicBody, position: Vec2::new(400.0, 200.0), velocity: Vec2::new(0.0, 0.0), restitution: 0.75, mass: 1.0, gravity_scale: 5.0};
    world.add_body(circle_body_def);*/

    let circle_shape2 = box2d::shape::shape::Shape::CircleShape { center: Vec2::new(0.0, 0.0), radius: 20.0 };
    let circle_body_def2 = BodyDef{shape: circle_shape2, body_type: BodyType::DynamicBody, position: Vec2::new(300.0, 200.0), velocity: Vec2::new(0.0, 0.0), restitution: 0.75, mass: 1.0, gravity_scale: 10.0};
    world.add_body(circle_body_def2);

    let chain_line_shape = box2d::shape::shape::Shape::ChainLineShape{ points: vec![Vec2::new(-200.0, -50.0), Vec2::new(-100.0, 0.0), Vec2::new(100.0, 0.0), Vec2::new(200.0, -50.0)] };
    let chain_line_body_def = BodyDef{shape: chain_line_shape, body_type: BodyType::StaticBody, position: Vec2::new(400.0, 400.0), velocity: Vec2::new(0.0, 0.0), restitution: 1.0, mass: 0.0, gravity_scale: 1.0};
    world.add_body(chain_line_body_def);

    return world;
}
