#pragma once

#include "vec3.h"
#include "ray.h"
#include "hit.h"

class Sphere{

    public:
    vec3 origin;
    float radius;

    bool collide(Ray ray);
    const Hit collisionNormal(const Ray& ray, float mint, float maxt) const ;
    inline Sphere(vec3 o, float r): origin(o), radius(r){};
};