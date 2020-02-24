use minifb::Window;
use crate::objects::scene::Scene;
use crate::objects::ray::Ray;
use crate::maths::vector3f::Vector3f;
use crate::objects::intersection::Intersection;
use crate::objects::color4u8::Color4u8;

pub struct Application {
    pub buffers: Vec<u32>,
    pub window: Window,
    pub width: usize,
    pub height: usize
}

impl Application {

    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjust = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let sensor_x = (((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio * fov_adjust;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0) * fov_adjust;

        Ray {
            origin: Vector3f::zero(),
            direction: Vector3f::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }

    pub fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color4u8 {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.object.surface_normal(&hit_point);
        let direction_to_light = scene.light.direction.normalize() * (-1.0);
        let light_power = (Vector3f::dot(&surface_normal, &direction_to_light).max(0.0) * scene.light.intensity);
        let light_reflected = intersection.object.get_albedo() / std::f32::consts::PI;

        // Color components
        let color_r = intersection.object.get_color().r as f32 * scene.light.color.r as f32 * light_power * light_reflected;
        let color_g = intersection.object.get_color().g as f32 * scene.light.color.g as f32 * light_power * light_reflected;
        let color_b = intersection.object.get_color().b as f32 * scene.light.color.b as f32 * light_power * light_reflected;

        // Color as 4u8 (u32)
        // TODO: fix the range conversion
        Color4u8::from_f32s(color_r, color_g, color_b, 255.0)
    }

    pub fn render(app: &mut Application, scene: &Scene) {

        for i in 0..scene.width {
            for j in 0..scene.height {
                let ray = Application::create_prime(i as u32, j as u32, scene);
                match scene.trace(&ray) {
                    Some(inter) => {
                        app.buffers[j * app.width + i] = Application::get_color(scene, &ray, &inter).as_u32();
                    },
                    None => (),
                }
            }
        }
    }

}