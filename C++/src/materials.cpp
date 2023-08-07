#include "materials.h"


Ray materials::uniform_scatter(Hit hit, Ray r)
{
    vec3 target = hit.point + hit.normal +  vec3::random_unit_vec();
    return Ray(hit.point, (target - hit.point).unit_vector());
}

Ray materials::metallic(Hit h, Ray r)
{
    return Ray(h.point, r.direction.unit_vector().reflect(h.normal).unit_vector());
}

Ray materials::metallic_fuzzy03(Hit h, Ray r)
{
    return Ray(h.point, r.direction.unit_vector().reflect(h.normal) + vec3::random_unit_vec() * 0.3);
}
