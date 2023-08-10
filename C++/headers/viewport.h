#pragma once

#include "external_includes.h"
#include "scene.h"
#include "ray.h"
#include "vec3.h"
#include "ppm_writer.h"
#include "RGB.h"
#include "defines.h"

using Img = std::vector<std::vector<RGB_float>>;

class Viewport
{

public:
    float aspect_ratio;
    uint width, height;
    uint samples_per_pixel;
    uint max_reflections;
    float gamma = 2;

    inline Viewport(uint width, uint height, uint samples): width(width),
                                                            height(height),
                                                            aspect_ratio(width / float(height)),
                                                            samples_per_pixel(samples),
                                                            max_reflections(10) {}
    inline Viewport(uint width, uint height):   width(width),
                                                height(height),
                                                aspect_ratio(width / float(height)),
                                                samples_per_pixel(1),
                                                max_reflections(10) {}

    inline Viewport(uint width, uint height, uint samples, uint max_reflections) : width(width),
                                                             height(height),
                                                             aspect_ratio(width / float(height)),
                                                             samples_per_pixel(samples),
                                                             max_reflections(max_reflections) {}

    inline Viewport(uint width, float aspect_ratio, uint samples) : width(width),
                                                                    aspect_ratio(aspect_ratio),
                                                                    height(static_cast<int>(width / aspect_ratio)),
                                                                    samples_per_pixel(samples) {}
    
    inline Viewport(uint width, float aspect_ratio, uint samples, uint max_reflections) : width(width),
                                                                    aspect_ratio(aspect_ratio),
                                                                    height(static_cast<int>(width / aspect_ratio)),
                                                                    samples_per_pixel(samples),
                                                                    max_reflections(max_reflections) {}

    Img Render(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint, uint), const Scene &scene);
    Img Render_no_rand(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint, uint), const Scene &scene);
};