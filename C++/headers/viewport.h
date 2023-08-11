#pragma once

#include "external_includes.h"
#include "scene.h"
#include "ray.h"
#include "vec3.h"
#include "ppm_writer.h"
#include "RGB.h"
#include "defines.h"

using Img = std::vector<std::vector<RGB_float>>;

class Camera{
public:
    float vfov = 90;
    float aspect_ratio;
    uint width, height;
    vec3 pixel_delta_u, pixel_delta_v;
    vec3 pixel00_loc;
    vec3 u, v, w;
    vec3 origin= vec3(0,0,0), vup = vec3(0, 1, 0), direction = vec3(0, 0, -1);

    inline Camera(uint width, float aspect_ratio, float vfov, vec3 origin, vec3 vup, vec3 direction): 
        width(width),
        aspect_ratio(aspect_ratio),
        height(static_cast<uint>(width/aspect_ratio)),
        origin(origin),
        vup(vup.unit_vector()),
        direction(direction.unit_vector())
    {
        w = -this->direction;
        u = vup.cross(w).unit_vector();
        v = w.cross(u);
        auto h = tan(vfov * M_PI / 360);
        auto viewport_height = 2 * h;
        auto viewport_width = aspect_ratio * viewport_height;
        vec3 viewport_u = viewport_width * u;
        vec3 viewport_v = viewport_height * -v;
        pixel_delta_u = viewport_u / width;
        pixel_delta_v = viewport_v / height;
        auto viewport_upper_left = origin - w - viewport_u/2 - viewport_v/2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }
    inline Camera(uint width, float aspect_ratio, float vfov): 
        width(width),
        aspect_ratio(aspect_ratio),
        height(static_cast<uint>(width/aspect_ratio)),
        origin(vec3(0, 0, 0)),
        vup(vec3(0, 1, 0)),
        direction(vec3(0, 0, -1))
    {
        w = -this->direction;
        u = vup.cross(w).unit_vector();
        v = w.cross(u);
        auto h = tan(vfov * M_PI / 360);
        auto viewport_height = 2 * h;
        auto viewport_width = aspect_ratio * viewport_height;
        vec3 viewport_u = viewport_width * u;
        vec3 viewport_v = viewport_height * -v;
        pixel_delta_u = viewport_u / width;
        pixel_delta_v = viewport_v / height;
        auto viewport_upper_left = origin - w - viewport_u/2 - viewport_v/2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }
};

class Viewport{

public:
    uint samples_per_pixel;
    uint max_reflections;
    float gamma = 2;
    Camera cam = Camera(300, 1.5, 90, vec3(0,0,0), vec3(0, 1, 0), vec3(0, 0, -1));

    inline Viewport(uint samples): 
        samples_per_pixel(samples),
        max_reflections(10) {}
    inline Viewport():
        samples_per_pixel(1),
        max_reflections(10) {}
    inline Viewport(uint samples, uint max_reflections) : 

        samples_per_pixel(samples),
        max_reflections(max_reflections) {}
    inline Viewport(Camera cam, uint samples, uint max_reflections):
        cam(cam),
        samples_per_pixel(samples),
        max_reflections(max_reflections) {}
    
    Img Render(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint, uint), const Scene &scene);
    Img Render_no_rand(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint, uint), const Scene &scene);
};

