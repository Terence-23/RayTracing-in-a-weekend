#include "sphere.h"


bool Sphere::collide(Ray ray){
    vec3 oc = ray.origin - origin;
    auto a = ray.direction.dot(ray.direction);
    auto b = 2.0 * oc.dot(ray.direction);
    auto c = oc.dot(oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
}

vec3 Sphere::collisionNormal(Ray ray)
{
    vec3 oc = ray.origin - origin;
    auto a = ray.direction.dot(ray.direction);
    auto b = oc.dot(ray.direction);
    auto c = oc.dot(oc) - radius*radius;
    auto discriminant = b*b - a*c;
    if (discriminant < 0) return vec3(0,0,0);
    
    
    auto x1 = (-b - sqrt(discriminant)) / (a);
    auto x2 = (-b + sqrt(discriminant)) / (a);

    auto x = std::min(x1, x2);

    if (x < 0) return vec3(0,0,0);

    return (ray.at(x) - origin).unit_vector();
}
