#pragma once

#include"sphere.h"
#include"external_includes.h"


class Scene
{
private:
    /* data */
public:

    std::vector<Sphere> spheres;

    inline Scene(): spheres(){};
    ~Scene() = default;

};




