#include "vec3.h"
#include "ray.h"

class Sphere{

    public:
    vec3 origin;
    float radius;

    bool collide(Ray ray);
    inline Sphere(vec3 o, float r): origin(o), radius(r){};
};