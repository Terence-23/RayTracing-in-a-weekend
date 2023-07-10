#include "ray.h"

Ray::Ray(const vec3& origin, const vec3& direction): origin(origin), direction(direction)
{
}
Ray::Ray(const vec3& origin): origin(origin), direction(0, 0, 0)
{

}
Ray::Ray(): origin(0, 0, 0), direction(0,0,0)
{

}
vec3 Ray::at(double t) const{
    return origin + direction * t;
}