#include "sphere.h"


bool Sphere::collide(Ray ray){
    vec3 oc = ray.origin - origin;
    auto a = ray.direction.dot(ray.direction);
    auto b = 2.0 * oc.dot(ray.direction);
    auto c = oc.dot(oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
}