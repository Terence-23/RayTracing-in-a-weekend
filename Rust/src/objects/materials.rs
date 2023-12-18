use super::{Hit, JsonValue};
    use crate::vec3::{ray::Ray, vec3::Vec3};
    use rand::random;
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Material{
        pub metallicness: f32, 
        pub opacity: f32,
        pub ir: f32
    }
    impl Into<JsonValue> for Material{
        fn into(self) -> JsonValue {
            json::object! {
                metallicness: self.metallicness,
                opacity: self.opacity,
                ir: self.ir    
            }
        }
    }
    impl TryFrom<JsonValue> for Material{
        type Error = crate::viewport::errors::ParseError;

        fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
            Ok(
                Material { 
                    metallicness: match value["metallicness"].as_f32(){
                        Some(x) => x,
                        None => return Err(Self::Error{source: None})
                    }, 
                    opacity: match value["opacity"].as_f32(){
                        Some(x) => x,
                        None => return Err(Self::Error{source: None})
                    }, 
                    ir: match value["ir"].as_f32(){
                        Some(x) => x,
                        None => return Err(Self::Error{source: None})
                    }, 
                }
            )
        }
    }
    impl Material{
        pub fn new(metallicness: f32, opacity: f32, ir: f32) -> Self{
            Self { metallicness: metallicness, opacity: opacity, ir:ir }
        }
        pub fn new_m(metallicness: f32) -> Self{
            Self { metallicness: metallicness, opacity: 0.0, ir: 1.0 }
        }
        fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
            let mut cos_theta = (-uv).dot(n);
            if cos_theta > 1.0 {cos_theta = 1.0}
            let r_out_perp =   (uv + n * cos_theta) * etai_over_etat;
            let r_out_parallel = n * -(1.0 - r_out_perp.length2()).abs().sqrt();
            return r_out_perp + r_out_parallel;
        }
        fn reflectance(cosine: f32, ref_idx: f32) -> f32{
            // Use Schlick's approximation for reflectance.
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0*r0;
            return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
        }
    
        pub fn on_hit(&self, h :Hit, r: Ray) -> Ray{
            if self.opacity > 0.0{     
                let n;               
                let front_face = if r.direction.dot(h.normal) > 0.0 {
                    n = -h.normal;
                    false
                } else {
                    n = h.normal;
                    true
                };
                let refraction_ratio =  if front_face {1.0 / self.ir}else{self.ir};

                let unit_direction = r.direction.unit();
                let mut cos_theta = (-unit_direction).dot(n);
                if cos_theta > 1.0 {cos_theta = 1.0;}
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let reflectance = Self::reflectance(cos_theta, refraction_ratio);
                // eprintln!("ff: {} can refract: {} ref_ratio: {}", front_face, !cannot_refract, refraction_ratio);
                let direction = if cannot_refract || reflectance > random::<f32>(){
                    // eprintln!("reflect");
                    unit_direction.reflect(n)
                }else{
                    // eprintln!("ud: {:?} hn: {:?}", unit_direction, n);
                    Self::refract(unit_direction, n, refraction_ratio)
                };

                return Ray::new_with_time(h.point, direction, r.time);

            }
            // eprintln!("reflect");
            let sc = diffuse(h, r).direction * (1.0 - self.metallicness);
            let mut reflect = Ray::new(h.point, r.direction.unit().reflect(h.normal));
            reflect.direction = reflect.direction * self.metallicness + sc;
            return reflect;
        }
    }

    pub const METALLIC_M: Material = Material{metallicness: 1.0, opacity: 0.0, ir: 1.0};
    pub const SCATTER_M: Material = Material{metallicness: 0.0, opacity: 0.0, ir: 1.0};
    pub const FUZZY3_M: Material = Material{metallicness: 0.7, opacity: 0.0, ir: 1.0};
    pub const GLASS_M: Material = Material{metallicness: 1.0, opacity: 1.0, ir:1.50};
    pub const GLASSR_M: Material = Material{metallicness: 1.0, opacity: 1.0, ir: 1.0/GLASS_M.ir};
    pub const EMPTY_M: Material = SCATTER_M;
    fn empty(_hit: Hit, r:Ray) -> Ray{
        Ray{origin:Vec3 { x: 0.0, y: 0.0, z: 0.0 }, direction:Vec3 { x: 0.0, y: 0.0, z: 0.0 }, time: r.time}
    }
    fn diffuse(hit :Hit, r: Ray) -> Ray{
        // println!("diff");
        let target = hit.normal +  Vec3::random_unit_vec();
        if target.close_to_zero(){
            return Ray{origin: hit.point, direction: hit.normal, time:r.time};
        }
        return Ray{origin: hit.point, direction: target, time: r.time};
    }
    fn metallic(hit :Hit, r: Ray) -> Ray{
        Ray{origin: hit.point, direction: r.direction.unit().reflect(hit.normal), time:r.time}
    }
    fn metal_fuzzy03(hit :Hit, r: Ray) -> Ray{
        Ray{origin: hit.point, direction: (r.direction.unit().reflect(hit.normal) + Vec3::random_unit_vec() * 0.3).unit(), time:r.time}
    }
