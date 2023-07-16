#include "materials.h"


Ray materials::uniform_scatter(Hit hit)
{
    vec3 target = hit.point + hit.normal +  vec3::random_unit_vec();
    return Ray(hit.point, target - hit.point);
}


