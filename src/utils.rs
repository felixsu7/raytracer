use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub fn lambertian_sphere(
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    r: f64,
    g: f64,
    b: f64,
) -> Box<Sphere> {
    Box::new(Sphere::new(
        Vec3::new(x, y, z),
        radius,
        Material::Lambertian {
            albedo: Vec3::new(r, g, b),
        },
    ))
}

pub fn metal_sphere(
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    r: f64,
    g: f64,
    b: f64,
    fuzz: f64,
) -> Box<Sphere> {
    Box::new(Sphere::new(
        Vec3::new(x, y, z),
        radius,
        Material::Metal {
            albedo: Vec3::new(r, g, b),
            fuzz,
        },
    ))
}

pub fn glass_sphere(
    x: f64,
    y: f64,
    z: f64,
    radius: f64,
    r: f64,
    g: f64,
    b: f64,
    ir: f64,
) -> Box<Sphere> {
    Box::new(Sphere::new(
        Vec3::new(x, y, z),
        radius,
        Material::Glass {
            albedo: Vec3::new(r, g, b),
            ir,
        },
    ))
}

pub fn something_sphere(x: f64, y: f64, z: f64, radius: f64) -> Box<Sphere> {
    Box::new(Sphere::new(
        Vec3::new(x, y, z),
        radius,
        Material::Something {},
    ))
}
