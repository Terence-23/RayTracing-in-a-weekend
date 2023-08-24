#pragma once

#include"external_includes.h"
#include"defines.h"
#include"vec3.h"
#include"ray.h"
#include"hit.h"

namespace materials
{
    using material = Ray(*)(Hit, Ray);

    Ray uniform_scatter(Hit hit, Ray r);
    inline Ray empty(Hit hit, Ray r){return Ray(vec3(0,0,0), vec3(0,0,0));}
    Ray metallic(Hit, Ray);
    Ray metallic_fuzzy03(Hit, Ray);

    inline vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
        auto cos_theta = fmin((-uv).dot(n), 1.0);
        vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
        vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length2())) * n;
        return r_out_perp + r_out_parallel;
    }

    class Material{
    private:
        inline static double reflectance(double cosine, double ref_idx) {
            // Use Schlick's approximation for reflectance.
            auto r0 = (1-ref_idx) / (1+ref_idx);
            r0 = r0*r0;
            return r0 + (1-r0)*pow((1 - cosine),5);
        }
    public:
        float metallicness, opacity, ir;
        Material() = default;
        inline Material(float metallicness, float opacity, float ir): metallicness(metallicness), opacity(opacity), ir(ir){}
        inline Material(float metallicness): metallicness(metallicness), opacity(0), ir(1){}

        inline std::string to_json(){
            return "{\"metallicness\":" + std::to_string(metallicness) + ",\"opacity\":" + std::to_string(opacity) + ",\"ir\":" + std::to_string(ir) + '}'; 
        }
        static inline Material from_json(std::string json){
            std::cout << "Material: " << json << '\n';
            json = remove_whitespace(json);
            size_t l = 0;
            if(json[l] != '{'){
                l = json.find('{');
            }
            size_t r = find_closing(json, l, json.size());
            
            float metallicness = -1, opacity = -1, ir = -1;

            // std::cout << "l: " << l << " r: " << r << '\n';
            for(size_t i = l+1; i < r; ++i){
                if (json[i] == ':'){
                    if (json.substr(l+1, i - l - 1) == "\"metallicness\""){
                        size_t c = json.find_first_of(",}", ++i);
                        metallicness = atof(json.substr(i, c - i).c_str());
                        // std::cout << "Found metallicness: " << metallicness << '\n';

                    } else if (json.substr(l+1, i - l - 1) == "\"opacity\""){

                        size_t c = json.find_first_of(",}", ++i);
                        opacity = atof(json.substr(i, c - i).c_str());
                        // std::cout << "Found opacity: " << opacity << '\n';

                    } else if (json.substr(l+1, i - l - 1) == "\"ir\""){

                        size_t c = json.find_first_of(",}", ++i);
                        ir = atof(json.substr(i, c - i).c_str());
                        // std::cout << "Found ir: " << ir << '\n';

                    }
                }
                else if (json[i] == ','){
                    l = i;
                    // std:: cerr << "New l: " << l << '\n';
                }
            }
            
            std::cout << "Material: " << metallicness << ' ' << opacity << ' ' << ir << '\n';
            return Material(metallicness, opacity, ir);
        }

        inline Ray onHit(Hit h, Ray r) const {
            if (0 < opacity){

                bool front_face;
                if (r.direction.dot(h.normal) > 0.0) {
                    front_face = false;
                    h.normal = -h.normal;
                } else {
                    front_face = true;
                }
                
                double refraction_ratio =  front_face ? (1.0/ir) : ir;


                vec3 unit_direction = r.direction.unit_vector();
                double cos_theta = fmin((-unit_direction).dot(h.normal), 1.0);
                double sin_theta = sqrt(1.0 - cos_theta*cos_theta);

                bool cannot_refract = refraction_ratio * sin_theta > 1.0;
                vec3 direction;
                // std::cerr << "ff: " << front_face <<" can refract: " << !cannot_refract << " ref_ratio: " << refraction_ratio << '\n';
                if (cannot_refract){// || reflectance(cos_theta, refraction_ratio) * opacity > random_double())
                    direction = unit_direction.reflect(h.normal);
                    // std::cerr << "reflect" << '\n';
                }else{

                    // std::cerr << "ud "<< unit_direction << " hn " << h.normal << '\n';
                    direction = refract(unit_direction, h.normal, refraction_ratio);
                }
                
                return Ray(h.point, direction);
            }
            // std::cerr << "reflect" << '\n';
            vec3 sc = materials::uniform_scatter(h, r).direction * (1.0 - this->metallicness);
            Ray reflect = materials::metallic(h,r);
            reflect.direction = reflect.direction * this->metallicness + sc;
            return reflect;
        }

        inline bool operator==(const Material& v)const{
            return metallicness == v.metallicness && opacity == v.opacity && ir == v.ir;
        }
    };

    const Material metallicM = Material(1.0);
    const Material scatterM = Material(0.0);
    const Material fuzzy3 = Material(0.7);
    const Material glass = Material(1, 1, 1.5);
    const Material glassR = Material(1, 1, 1/1.5);
 
    const Material empty_mat = scatterM;

    
}




