#include"viewport.h"
#include "../lib/tqdm.cpp/include/tqdm/tqdm.h"

Img Viewport::Render(RGB_float (*ray_color)(const Ray& r, const Scene& scene), const Scene& scene){
    
    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    // std::cout << "\nH: " << height << " W: " << width << " W/H: " << double(width) / height << "\nAspect ratio: " << aspect_ratio << '\n';

    vec3 origin(0, 0, 0);
    vec3 horizontal(viewport_width, 0, 0), vertical(0, viewport_height, 0);
    vec3 lower_left_corner = origin - horizontal / 2 - vertical / 2 - vec3(0, 0, focal_length);

    std::vector<std::vector<RGB_float>> img(height);
    for (int j : tqdm::range(height))
    {
        std::vector<RGB_float> row(width);

        for (int i = 0; i < width; ++i)
        {
            auto u = double(i) / (width - 1);
            auto v = double(height - 1 - j) / (height - 1);
            Ray r(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            row[i] = ray_color(r, scene);
        }

        img[j] = row;
    }
    return img;
}