use std::rc::Rc;
extern crate nalgebra_glm as glm;

use nalgebra::{Vector2, Vector3};

use serde_json::{Value, from_value};

use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::{random_in_unit_sphere, reflect};

pub struct HitInfo
{   
    /// Ray parameter for the hit
    pub t: f32, 
    /// Hit position            
    pub p: Vector3<f32>, 
    /// Geometric normal   
    pub gn: Vector3<f32>,   
    /// Interpolated shading normal
    pub sn: Vector3<f32>,  
    /// UV texture coordinates 
    pub uv: Vector2<f32>,
    /// Material at the hit point
    pub mat: Rc<dyn Material>
}

// TODO : CHANGE THIS< THIS IS HORRIBLE
impl HitInfo {
    pub fn empty() -> HitInfo{
        HitInfo { 
            t: -1., 
            p: Default::default(), 
            gn: Default::default(), 
            sn: Default::default(), 
            uv: Default::default(), 
            mat: Rc::new(Lambertian{albedo:Vector3::x()})
        }
    }
}
// /// Data record for conveniently querying and sampling emitters
// pub struct EmitterRecord
// {
//     /// Origin point from which we sample the emitter
//     o: Vector3<f32>,
//     /// Direction vector from 'o' to 'hit.p
//     wi: Vector3<f32>,  
//     /// Solid angle density wrt. 'o'
//     pdf: f32, 
//     /// Hit information at the sampled point
//     hit: HitInfo
// }

////////////////////////
/// SURFACE 
///////////////////////

pub trait Surface {
    // fn build_surface();
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
    // fn bounds();
    // fn sample(emit_rec: &EmitterRecord, rv: &Vector2<f32>) -> Vector3<f32>;
    // fn pdf(emit_rec: &EmitterRecord, rv: &Vector2<f32>) -> f32;
    // fn is_emissive() -> bool;
}


pub struct Sphere {
    pub radius: f32,
    pub transform: Transform,
    pub material: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(radius: f32, material: Rc<dyn Material>) -> Sphere{
        Sphere{
            radius: radius,
            transform: Default::default(),
            material: material
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // put ray into sphere frame
        let ray_transformed = self.transform.inverse().ray(ray);

        let oc = ray_transformed.origin;
    
        let a = ray_transformed.direction.norm_squared();
        let half_b =  oc.dot(&ray_transformed.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        
        if discriminant < 0.0{
            return None;
        }
        // Find the nearest root that lies in the acceptable range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < ray_transformed.mint || root > ray_transformed.maxt {
            root = (-half_b + discriminant_sqrt) / a;
            if root < ray_transformed.mint || root > ray_transformed.maxt {
                return None;
            }
        }

        let p_sphere_frame = ray_transformed.at(root);
        // put point and normal back into the world frame
        let p = self.transform.point(&p_sphere_frame);
        let n = self.transform.normal(&(p_sphere_frame / self.radius));
        
        let hit = HitInfo{
            t: root,
            p: p,
            gn: n,
            sn: n,
            uv: Vector2::new(0.0, 0.0),
            mat: Rc::clone(&self.material)
        };
        Some(hit)
    }
}

pub struct SurfaceGroup{
    pub surfaces: Vec<Rc<dyn Surface>>,
    // bounds: Box3
}

// TODO : CHANGE THIS< THIS IS HORRIBLE
impl Surface for SurfaceGroup {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut ray_mut: Ray = (*ray).clone();
        let mut hit_anything = false;
        let mut hit_out = HitInfo::empty();

        for surface in &self.surfaces{
            if let Some(hit) = surface.intersect(&ray_mut){
                // println!("found something");
                hit_anything = true;
                ray_mut.maxt = hit.t;
                hit_out = hit;
                // return Some(hit);
            }
        }

        if hit_anything{
            // Some(hit)
            Some(hit_out)
        }
        else{
            None
        }
        
    }
}

impl SurfaceGroup {
    pub fn new() -> SurfaceGroup{
        SurfaceGroup{surfaces: Vec::new()}
    }
    pub fn add_child(&mut self, surface: Rc<dyn Surface>)
    {
        self.surfaces.push(surface.clone())
    }

    pub fn add_to_parent(&self)
    { 

    }
}
//////////////////////////////
/// MATERIAL
//////////////////////////////

pub trait Material {
    ///Compute the scattered direction scattered at a surface hitpoint.
    ///The base Material does not scatter any light, so it simply returns false.
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vector3<f32>, Ray)>;
    
    // /// Compute the amount of emitted light at the surface hitpoint.
    // /// The base Material class does not emit light, so it simply returns black.
    // fn emmitted(&self, ray: &Ray, hit: &HitInfo);
    
    // /// Return whether or not this Material is emissive.
    // /// 
    // /// This is primarily used to create a global list of emitters for sampling.
    // fn is_emissive(&self) -> bool;
    
    // /// Evaluate the material response for the given pair of directions.
    // /// 
    // /// For non-specular materials, this should be the BSDF multiplied by the
    // /// cosine foreshortening term.
    // /// Specular contributions should be excluded.
    // fn eval(&self, wi: &Vector3<f32>, scattered: &Vector3<f32>, hit: &HitInfo) -> Vector2<f32>;
    
    // /// Sample a scattered direction at the surface hitpoint \p hit.
    // /// 
    // /// If it is not possible to evaluate the pdf of the material (e.g.\ it is
    // /// specular or unknown), then set \c srec.is_specular to true, and populate
    // /// \c srec.wo and \c srec.attenuation just like we did previously in the
    // /// #scatter() function. This allows you to fall back to the way we did
    // /// things with the #scatter() function, i.e.\ bypassing #pdf()
    // /// evaluations needed for explicit Monte Carlo integration in your
    // /// #Integrator, but this also precludes the use of MIS or mixture sampling
    // /// since the pdf is unknown.
    // fn sample(&self) -> bool;

    // /// Compute the probability density that #sample() will generate \c scattered (given \c wi).
    // fn pdf(&self, wi: &Vector3<f32>, scattered: &Vector3<f32>, hit: &HitInfo) -> f32;
}

pub struct  Lambertian{
   pub albedo: Vector3<f32>
}

impl Material for Lambertian{
    fn scatter(&self, _r_in: &Ray, hit: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        let mut rng = rand::thread_rng();
        let mut scatter_direction = hit.sn + random_in_unit_sphere(&mut rng).normalize();
        
        // Catch degenerate scatter direction
        const EPSILON: f32= 1.0e-6;
        if scatter_direction.norm_squared() < EPSILON{
            scatter_direction = hit.sn;
        }
        
        let attenuation = self.albedo;
        let ray_out = Ray::new(hit.p, scatter_direction.normalize());

        Some((attenuation, ray_out))
    }
}


pub struct Metal{
    pub albedo: Vector3<f32>,
    pub roughness: f32
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        let mut rng = rand::thread_rng();
        
        let reflected = reflect(&r_in.direction, &hit.sn);
        
        let scatter_direction = reflected + self.roughness * random_in_unit_sphere(&mut rng).normalize();
        
        if scatter_direction.dot(&hit.sn) < 0.0 {
            return None;
        }
        let attenuation = self.albedo;
        let ray_out = Ray::new(hit.p, scatter_direction.normalize());

        Some((attenuation, ray_out))

    }
}

pub fn create_material(material_json: Value) ->Rc<dyn Material>{
    
    let read_vector3 = |v: &Value| from_value::<Vector3<f32>>(v.clone()).unwrap();
    let read = |s: &str, default| material_json.get(s).map_or(default, read_vector3);
    
    let type_material = material_json.get("type").expect("material should have a type");

    if type_material == "lambertian"
    {
        let albedo = material_json.get("albedo").unwrap().clone();
        if albedo.is_number(){
            let albedo_number: f32 = from_value(albedo).unwrap();
            return Rc::new(Lambertian{albedo: Vector3::new(albedo_number, albedo_number, albedo_number)});
        }

        let albedo = read("albedo", Vector3::zeros());
        Rc::new(Lambertian{albedo: albedo})
    }
    else if type_material == "metal"
    {
        let albedo_v = material_json.get("albedo").unwrap().clone();
        let albedo: Vector3<f32>;
        if albedo_v.is_number(){
            let albedo_number: f32 = from_value(albedo_v).unwrap();
            albedo = Vector3::new(albedo_number, albedo_number, albedo_number);
        } else{
            albedo = read("albedo", Vector3::zeros());
        }
        let roughness = material_json.get("roughness").map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
        Rc::new(Metal{albedo: albedo, roughness: roughness})
    }
    else 
    {
        panic!("This type is not yet implemented")
    }
}