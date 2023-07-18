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
    Ray (*material)(Hit, Ray);
    vec3 col_mod;
    

    bool collide(Ray ray);
    const Hit collisionNormal(const Ray& ray, float mint, float maxt) const ;
    inline Sphere(vec3 o, float r): origin(o), radius(r), material(materials::empty), col_mod(vec3(1,1,1)){};
    inline Sphere(vec3 o, float r, materials::material mat, vec3 col_mod): origin(o), radius(r), material(mat), col_mod(col_mod){};
};