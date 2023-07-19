#pragma once

#include"external_includes.h"
#include"defines.h"
#include"vec3.h"
#include"ray.h"
#include"hit.h"

namespace materials
{
    using material = Ray(*)(Hit, Ray);

    Ray uniform_scatter(Hit hit, Ray r);
    inline Ray empty(Hit hit, Ray r){return Ray(vec3(0,0,0), vec3(0,0,0));}
    Ray metalic(Hit, Ray);
    Ray metalic_fuzzy03(Hit, Ray);
}




