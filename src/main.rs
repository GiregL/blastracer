use minifb::{Window, WindowOptions, Key, Scale};

pub mod maths;
pub mod core;
pub mod objects;

use crate::core::application::Application;
use crate::objects::scene::Scene;
use crate::objects::ray::Ray;
use crate::maths::vector3f::Vector3f;
use crate::objects::sphere::Sphere;
use crate::objects::color4u8::Color4u8;
use crate::objects::plane::Plane;
use crate::objects::light::DirectionalLight;

const WIDTH: usize = 1080;
const HEIGHT: usize = 720;


fn main() {

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Blastracer", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let mut app: Application = Application {
        buffers: buffer,
        window,
        width: WIDTH,
        height: HEIGHT
    };

    let scene: Scene = Scene {
        width: app.width,
        height: app.height,
        fov: 70.0,
        shapes: vec![
            Box::new(Sphere {
                center: Vector3f::new(0.0, 0.0, 10.0),
                radius: 3.0,
                color: Color4u8::new(255, 0, 0, 255),
                albedo: 1.0
            }),
            Box::new(Sphere {
                center: Vector3f::new(1.0, 0.25, 5.0),
                radius: 0.5,
                color: Color4u8::new(0, 255, 0, 255),
                albedo: 1.0
            }), Box::new(Plane {
                p0: Vector3f::new(0.0, -0.5, 0.0),
                normal: Vector3f::new(0.0, -1.0, 0.0),
                color: Color4u8::new(50, 50, 50, 255),
                albedo: 1.0
            })
        ],
        light: DirectionalLight {
            direction: Vector3f::new(0.0, -3.0, 2.0),
            color: Color4u8::new(255, 255, 255, 255),
            intensity: 9.0
        }
    };

    Application::render(&mut app, &scene);

    while app.window.is_open() && !app.window.is_key_down(Key::Escape) {

        app.window.update_with_buffer(&app.buffers, app.width, app.height).unwrap();
    }
}
