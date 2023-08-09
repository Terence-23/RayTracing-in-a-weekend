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

    if (x1 < mint) x1 = x2; 
    if (x1 < mint || x1 > maxt) return NO_HIT;

    Hit hit = Hit(x1, (ray.at(x1) - origin).unit_vector(), ray.at(x1));
    Ray next = this->material.onHit(hit, ray);
    if (next.direction.near_zero()){
        next = hit.normal;
    }
    hit.next = next;
    hit.col_mod =this->col_mod;
    if (std::isnan(hit.t)) throw -1;
    return hit;
}
