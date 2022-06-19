use crate::{
    la::{Vec3f},
    raytracing::Ray,
    raytracing::{Hittable},
    raytracing::materials::{Dielectric,Lambertian,Material,Metal},
    models::objects::Sphere, 
    scene::{RayTracingScene,SceneTrait},
};
use std::sync::Arc;
use rand::{random, Rng};

pub fn random_color() -> Vec3f {
    random_color_in_range(0.0,1.0)
}

pub fn random_color_in_range(min: f32, max: f32) -> Vec3f {
    let gen_range = || -> f32 { rand::thread_rng().gen_range(min,max) };
    Vec3f(gen_range(), gen_range(), gen_range())
}

pub fn random_in_unit_sphere() -> Vec3f {
    loop {
        let p = 2.0 * random::<Vec3f>() - Vec3f(1.0, 1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_disk_vector() -> Vec3f {
    let gen_range = || -> f32 { rand::thread_rng().gen_range(-1.0,1.0) };
    loop {
        let p = Vec3f(gen_range(), gen_range(), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3f {
    loop {
        let u = 2.0 * random::<f32>() - 1.0;
        let v = 2.0 * random::<f32>() - 1.0;
        let r2 = u*u+v*v;
        if r2 < 1.0 {
            let x = 2.0 * u * (1.0 - r2).sqrt();
            let y = 2.0 * v * (1.0 - r2).sqrt();
            let z = 1.0 - 2.0 * r2;
            return Vec3f(x,y,z);
        }
    }
}

pub fn random_hemisphere_vector(normal: &Vec3f) -> Vec3f {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        - in_unit_sphere
    }
}

pub fn ray_color(ray: &Ray, world: &RayTracingScene, depth: usize) -> Vec3f {
    const WHITE: Vec3f= Vec3f(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3f = Vec3f(0.5, 0.7, 1.0);
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Vec3f(0.0, 0.0, 0.0);
    }

    if let Some(mut rec) = world.hit(0.001, f32::MAX, ray) {
        let material = rec.material.unwrap();
        rec.material = None;
        return if let Some(scattered) = material.scatter(&ray, &rec) {
            scattered.attenuation * ray_color(&scattered.ray, &world, depth - 1)
        } else {
            Vec3f(0.0, 0.0, 0.0)
        };
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}

pub fn random_scene() -> RayTracingScene {
    let mut world = RayTracingScene::new();
    let ground_material = Arc::new(Lambertian::new(Vec3f(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere {
        center: Vec3f(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    let random_double_in_range =
        |min: f32, max: f32| -> f32 { rand::thread_rng().gen_range(min,max) };
    let random_double = || -> f32 { random_double_in_range(0.0, 1.0) };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3f(
                a as f32 + 0.9 * random_double(),
                0.2,
                b as f32 + 0.9 * random_double(),
            );
            if (center - Vec3f(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_color() * random_color();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_color_in_range(0.5, 1.0);
                    let fuzz = random_double_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: sphere_material,
                }));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere {
        center: Vec3f(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2 = Arc::new(Lambertian::new(Vec3f(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere {
        center: Vec3f(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3 = Arc::new(Metal::new(Vec3f(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere {
        center: Vec3f(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    world
}
