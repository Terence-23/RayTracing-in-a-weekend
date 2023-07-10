#include "sphere.h"

bool Sphere::collide(Ray ray){
    vec3 oc = ray.origin - origin;
    auto a = ray.direction.dot(ray.direction);
    auto b = 2.0 * oc.dot(ray.direction);
    auto c = oc.dot(oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
}

const Hit Sphere::collisionNormal(const Ray& ray, float mint, float maxt) const 
{
    vec3 oc = ray.origin - origin;
    auto a = ray.direction.dot(ray.direction);
    auto b = oc.dot(ray.direction);
    auto c = oc.dot(oc) - radius*radius;
    auto discriminant = b*b - a*c;
    if (discriminant < 0) return NO_HIT;
    
    
    auto x1 = (-b - sqrt(discriminant)) / (a);
    auto x2 = (-b + sqrt(discriminant)) / (a);

    auto x = std::min(x1, x2);

    if (x < mint || x > maxt) return NO_HIT;

    return Hit(x, (ray.at(x) - origin).unit_vector(), ray.at(x));
}
