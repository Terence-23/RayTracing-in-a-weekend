#pragma once

#include"external_includes.h"
#include"scene.h"
#include"ray.h"
#include"vec3.h"
#include"ppm_writer.h"
#include"RGB.h"

using Img = std::vector<std::vector<RGB_float>>;

class Viewport{

public:

    float aspect_ratio;
    uint width, height;

    inline Viewport(uint width, uint height): width(width), height(height), aspect_ratio(width/float(height)){}
    inline Viewport(uint width, float aspect_ratio): width(width), aspect_ratio(aspect_ratio), height(static_cast<int>(width/aspect_ratio)){}

    Img Render(RGB_float (*ray_color)(const Ray& r, const Scene& scene), const Scene& scene);

};