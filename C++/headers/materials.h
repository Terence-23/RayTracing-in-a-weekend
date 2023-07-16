#pragma once

#include"external_includes.h"
#include"defines.h"
#include"vec3.h"
#include"ray.h"
#include"hit.h"

namespace materials
{
    using material = Ray(*)(Hit);

    Ray uniform_scatter(Hit hit);
    inline Ray empty(Hit hit){return Ray(vec3(0,0,0), vec3(0,0,0));}
} // namespace materials




