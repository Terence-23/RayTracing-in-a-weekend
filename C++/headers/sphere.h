#pragma once

#include "vec3.h"
#include "ray.h"
#include "hit.h"
#include "materials.h"
#include "RGB.h"

class Sphere{

    public:
    vec3 origin;
    float radius;
    materials::Material material;
    vec3 col_mod;
    

    bool collide(Ray ray);
    const Hit collisionNormal(const Ray& ray, float mint, float maxt) const ;
    inline Sphere(): origin(vec3(0,0,0)), radius(0), material(materials::empty_mat), col_mod(vec3(1,1,1)){};
    inline Sphere(vec3 o, float r): origin(o), radius(r), material(materials::empty_mat), col_mod(vec3(1,1,1)){};
    inline Sphere(vec3 o, float r, materials::Material mat, vec3 col_mod): origin(o), radius(r), material(mat), col_mod(col_mod){};

    inline bool operator==(const Sphere& v)const{
        return origin == v.origin && radius == v.radius && col_mod == v.col_mod && material == v.material;
    }


    inline std::string to_json(){
        return "{\"origin\":" + origin.to_json() + ",\"radius\":" + std::to_string(radius) + ",\"material\":" + material.to_json() + ",\"col_mod\":" + col_mod.to_json() +"}";
    }
    static inline Sphere from_json(std::string json){
        std::cout << "Sphere: " << json << '\n';
        json = remove_whitespace(json);
        size_t l = 0;
        if(json[l] != '{'){
            l = json.find('{');
        }
        size_t r = find_closing(json, l, json.size());
        vec3 origin, col_mod;
        float radius;
        materials::Material material;


        std::cout << "l: " << l << " r: " << r << '\n';
        for(size_t i = l+1; i < r; ++i){
            if (json[i] == ':'){
                std::cerr << "Colon at: " << i << '\n';
                std::cout << "Key: " << json.substr(l+1, i - l - 1) << '\n';
                if (json.substr(l+1, i - l - 1) == "\"origin\""){
                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    auto c = find_closing(json, ++i, r);
                    // std::cerr << json.substr(i, c - i) << '\n';
                    origin = vec3::from_json(json.substr(i, c - i + 1));
                    std::cout << "Found origin: " << json.substr(i, c - i + 1) << '\n';
                    i = l = c;

                } else if (json.substr(l+1, i - l - 1) == "\"radius\""){

                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    size_t c = json.find(',', ++i);
                    // std::cout << c << ' ' << json[c] << '\n';
                    // std::cerr << json.substr(i, c - i) << '\n';
                    radius = atof(json.substr(i, c - i).c_str());
                    std::cout << "Found radius: " << radius << '\n';
                    i = l = c;

                } else if (json.substr(l+1, i - l - 1) == "\"material\""){
                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    auto c = find_closing(json, ++i, r);
                    // std::cerr << json.substr(i, c - i) << '\n';
                    material = materials::Material::from_json(json.substr(i, c - i + 1));
                    std::cout << "Found material: " << json.substr(i, c - i + 1) << '\n';
                    i = l = c;

                } else if (json.substr(l+1, i - l - 1) == "\"col_mod\""){
                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    auto c = find_closing(json, ++i, r);
                    std::cout << json.substr(i, c - i) << '\n';
                    col_mod = vec3::from_json(json.substr(i, c - i + 1));
                    std::cout << "Found color: " << json.substr(i, c - i + 1) << '\n';
                    i = l = c;
                }
            }
            if (json[i] == ','){
                l = i;
                std:: cerr << "New l: " << l << '\n';
            }
            
        }
        std::cout << "Return sphere\n";
        return Sphere(origin, radius, material, col_mod);
    }
    friend std::ostream & operator <<( std::ostream & os, const Sphere & v ){    
        os << "Origin: " << v.origin << ", radius: " << v.radius << ", color: " << v.col_mod;
        return os;
    }
};