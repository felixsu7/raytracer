use crate::hittable::Intersection;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::random;

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Glass { albedo: Vec3, ir: f64 },
    Something {},
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}

pub fn scatter(
    material: Material,
    ray: &Ray,
    intersection: &Intersection,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    match material {
        Material::Lambertian { albedo } => lambertian(intersection, attenuation, scattered, albedo),
        Material::Metal { albedo, fuzz } => {
            metal(ray, intersection, attenuation, scattered, albedo, fuzz)
        }
        Material::Glass { albedo, ir } => {
            glass(ray, intersection, attenuation, scattered, albedo, ir)
        }
        Material::Something {} => something(ray, intersection, attenuation, scattered),
    }
}

fn lambertian(
    intersection: &Intersection,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    albedo: Vec3,
) -> bool {
    let mut scatter_direction = intersection.normal + Vec3::random_unit_vector();

    if scatter_direction.near_zero() {
        scatter_direction = intersection.normal;
    }

    *scattered = intersection.point.with_direction(scatter_direction);
    *attenuation = albedo;
    true
}

fn metal(
    ray: &Ray,
    intersection: &Intersection,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    albedo: Vec3,
    fuzz: f64,
) -> bool {
    let reflected = reflect(&ray.direction.unit_vector(), &intersection.normal);
    *scattered = intersection
        .point
        .with_direction(reflected + fuzz * Vec3::random_unit_vector());
    *attenuation = albedo;
    scattered.direction.dot(&intersection.normal) > 0.0
}

fn glass(
    ray: &Ray,
    intersection: &Intersection,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
    albedo: Vec3,
    ir: f64,
) -> bool {
    let refraction_ratio = if intersection.front_face {
        1.0 / ir
    } else {
        ir
    };
    let unit_direction = ray.direction.unit_vector();
    let cos_theta = unit_direction.inverse().dot(&intersection.normal).min(1.0);
    let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

    let cannot_refract = refraction_ratio * sin_theta > 1.0;

    if cannot_refract || reflectance(cos_theta, refraction_ratio) > random::<f64>() {
        *scattered = intersection
            .point
            .with_direction(reflect(&unit_direction, &intersection.normal));
    } else {
        *scattered = intersection.point.with_direction(refract(
            &unit_direction,
            &intersection.normal,
            refraction_ratio,
        ));
    }

    *attenuation = albedo;

    true
}

pub fn something(
    ray: &Ray,
    intersection: &Intersection,
    attenuation: &mut Vec3,
    scattered: &mut Ray,
) -> bool {
    *attenuation = ray.direction;
    *scattered = intersection.point.with_direction(ray.direction);
    true
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * v.dot(n) * (*n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = uv.inverse().dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * (*n));
    let r_out_parallel = -(f64::sqrt(1.0 - r_out_perp.length_squared()).abs()) * *n;
    r_out_perp + r_out_parallel
}

fn reflectance(cos: f64, ir: f64) -> f64 {
    let r0 = (1.0 - ir) / (1.0 + ir);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cos, 5)
}
