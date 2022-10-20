extern crate nalgebra_glm as glm;
use enum_dispatch::enum_dispatch;
use glm::{Vec2, Vec3};

use crate::surfaces::surface::{HitInfo, ScatterRecord};

use crate::ray::Ray;

#[enum_dispatch]
pub trait Material {
    ///Compute the scattered direction scattered at a surface hitpoint.
    ///The base Material does not scatter any light, so it simply returns false.
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)>;

    /// Compute the amount of emitted light at the surface hitpoint.
    /// The base Material class does not emit light, so it simply returns black.
    fn emmitted(&self, ray: &Ray, hit: &HitInfo) -> Option<Vec3>;

    /// Return whether or not this Material is emissive.
    ///
    /// This is primarily used to create a global list of emitters for sampling.
    fn is_emissive(&self) -> bool;

    /// Evaluate the material response for the given pair of directions.
    ///
    /// For non-specular materials, this should be the BSDF multiplied by the
    /// cosine foreshortening term.
    /// Specular contributions should be excluded.
    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3;

    /// Sample a scattered direction at the surface hitpoint \p hit.
    ///
    /// If it is not possible to evaluate the pdf of the material (e.g.\ it is
    /// specular or unknown), then set \c srec.is_specular to true, and populate
    /// \c srec.wo and \c srec.attenuation just like we did previously in the
    /// #scatter() function. This allows you to fall back to the way we did
    /// things with the #scatter() function, i.e.\ bypassing #pdf()
    /// evaluations needed for explicit Monte Carlo integration in your
    /// #Integrator, but this also precludes the use of MIS or mixture sampling
    /// since the pdf is unknown.
    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &Vec2) -> Option<(ScatterRecord, bool)>;

    /// Compute the probability density that #sample() will generate \c scattered (given \c wi).
    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32;
}

use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::fresnel_blend::FresnelBlend;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;

// #[derive(Debug, PartialEq, Serialize, Deserialize)]

#[enum_dispatch(Material)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaterialType {
    Lambertian,
    Dielectric,
    Metal,
    DiffuseLight,
    FresnelBlend,
}
