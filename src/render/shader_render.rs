use std::mem;

use crate::{
    la::{Vec3f},
    shader::Shader,
    tga::{self},
    transform::{barycentric},
};



pub fn triangle(v1: &Vec3f, v2: &Vec3f, v3: &Vec3f, sh: &mut dyn Shader) {
    let z = Vec3f(v2.0, v2.1, v2.2)
        .sub(&Vec3f(v1.0, v1.1, v1.2))
        .cross(&Vec3f(v3.0, v3.1, v3.2).sub(&Vec3f(v1.0, v1.1, v1.2)));

    // clip space 
    // clip z<0
    if z.2 < 0.0 {
        return;
    }

    let x0 = vec![v1.0, v2.0, v3.0]
        .iter()
        .fold(&v1.0, |xmin, x| if xmin > x { x } else { xmin })
        .round() as i32;
    let y0 = vec![v1.1, v2.1, v3.1]
        .iter()
        .fold(&v1.1, |ymin, y| if ymin > y { y } else { ymin })
        .round() as i32;
    let x1 = vec![v1.0, v2.0, v3.0]
        .iter()
        .fold(&v1.0, |xmax, x| if xmax < x { x } else { xmax })
        .round() as i32;
    let y1 = vec![v1.1, v2.1, v3.1]
        .iter()
        .fold(&v1.1, |ymax, y| if ymax < y { y } else { ymax })
        .round() as i32;

    for y in y0..=y1 {
        for x in x0..=x1 {
            let bc = barycentric(v1, v2, v3, (x as f32, y as f32));
            sh.fragment(&bc);
        }
    }
}

pub fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    img: &mut tga::Image,
    color: tga::Color,
) {
    let dx = if x1 > x0 { x1 - x0 } else { x0 - x1 };
    let dy = if y1 > y0 { y1 - y0 } else { y0 - y1 };

    if dx > dy {
        if x1 < x0 {
            mem::swap(&mut x1, &mut x0);
            mem::swap(&mut y1, &mut y0);
        }
        for x in x0..=x1 {
            let t = ((x - x0) as f32) / ((x1 - x0) as f32);
            let y = (y0 as f32) * (1f32 - t) + (y1 as f32) * t;
            img.set_pixel(x as i32, y.round() as i32, color);
        }
    } else {
        if y1 < y0 {
            mem::swap(&mut x1, &mut x0);
            mem::swap(&mut y1, &mut y0);
        }
        for y in y0..=y1 {
            let t = ((y - y0) as f32) / ((y1 - y0) as f32);
            let x = (x0 as f32) * (1f32 - t) + (x1 as f32) * t;
            img.set_pixel(x.round() as i32, y as i32, color);
        }
    }
}
