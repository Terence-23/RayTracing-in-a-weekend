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
    inline Ray refract(Hit, Ray){};

    class Material{
    public:
        float metallicness, opacity;
        Material() = default;
        inline Material(float metallicness, float opacity): metallicness(metallicness), opacity(opacity){}
        inline Material(float metallicness): metallicness(metallicness), opacity(0){}

        inline Ray onHit(Hit h, Ray r) const {
            if (random_double() < opacity){
                return materials::refract(h, r);
            }
            vec3 sc = materials::uniform_scatter(h, r).direction * (1.0 - this->metallicness);
            Ray reflect = materials::metallic(h,r);
            reflect.direction = reflect.direction * this->metallicness + sc;
            return reflect;
        }

    };

    const Material metallicM = Material(1.0);
    const Material scatterM = Material(0.0);
    const Material fuzzy3 = Material(0.7);
    const Material empty_mat = scatterM;

    
}




