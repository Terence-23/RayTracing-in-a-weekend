#include "viewport.h"
#include "../lib/indicators/single_include/indicators/indicators.hpp"

Img Viewport::Render(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint depth, uint max_depth), const Scene &scene)
{

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    // std::cout << "\nH: " << height << " W: " << width << " W/H: " << double(width) / height << "\nAspect ratio: " << aspect_ratio << '\n';

    vec3 origin(0, 0, 0);
    vec3 horizontal(viewport_width, 0, 0), vertical(0, viewport_height, 0);
    vec3 lower_left_corner = origin - horizontal / 2 - vertical / 2 - vec3(0, 0, focal_length);

    std::vector<std::vector<RGB_float>> img(height);

    float inv_g = 1/this->gamma;
    using namespace indicators;
    indicators::ProgressBar bar{
        option::BarWidth{50},
        option::Start{"["},
        option::Fill{"="},
        option::Lead{">"},
        option::Remainder{"-"},
        option::End{"]"},
        option::PrefixText{"Rendering image "},
        option::ForegroundColor{Color::green},
        option::ShowElapsedTime{true},
        option::ShowRemainingTime{true},
        option::FontStyles{std::vector<FontStyle>{FontStyle::bold}},
        option::MaxProgress{height}
    };

    for (int j =0; j < height; j ++ )
    {
        std::vector<RGB_float> row(width);
        bar.set_progress(j);

        for (int i = 0; i < width; ++i)
        {
            RGB_float pixel_color = RGB_float(0, 0, 0);
            // set progress bar
            
            for (int s = 0; s < samples_per_pixel; s++)
            {
                auto u = (i + random_double()) / (width - 1);
                auto v = (height - 1 - j + random_double()) / (height - 1);
                Ray r(origin, lower_left_corner + horizontal * u + vertical * v - origin);
                if (std::isnan(r.direction.x) ) throw -1;
                pixel_color += ray_color(r, scene, 0, this->max_reflections);
            }
            // std::cerr << pixel_color / samples_per_pixel << ' ' << pixel_color <<' ' << samples_per_pixel << '\n';
            if (!col_in_range(pixel_color / samples_per_pixel)) throw -1;

            row[i] = (pixel_color / samples_per_pixel).gamma(inv_g);
        }

        img[j] = row;
    }
    bar.mark_as_completed();
    std::cout << "Done\n";
    return img;
}

Img Viewport::Render_no_rand(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint, uint), const Scene &scene)
{
    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    // std::cout << "\nH: " << height << " W: " << width << " W/H: " << double(width) / height << "\nAspect ratio: " << aspect_ratio << '\n';

    vec3 origin(0, 0, 0);
    vec3 horizontal(viewport_width, 0, 0), vertical(0, viewport_height, 0);
    vec3 lower_left_corner = origin - horizontal / 2 - vertical / 2 - vec3(0, 0, focal_length);

    std::vector<std::vector<RGB_float>> img(height);

    float inv_g = 1/this->gamma;
    using namespace indicators;
    // indicators::ProgressBar bar{
    //     option::BarWidth{50},
    //     option::Start{"["},
    //     option::Fill{"="},
    //     option::Lead{">"},
    //     option::Remainder{"-"},
    //     option::End{"]"},
    //     option::PrefixText{"Rendering image "},
    //     option::ForegroundColor{Color::green},
    //     option::ShowElapsedTime{true},
    //     option::ShowRemainingTime{true},
    //     option::FontStyles{std::vector<FontStyle>{FontStyle::bold}},
    //     option::MaxProgress{height}
    // };

    for (int j =0; j < height; j ++ )
    {
        std::vector<RGB_float> row(width);
        // bar.set_progress(j);

        for (int i = 0; i < width; ++i)
        {
            RGB_float pixel_color = RGB_float(0, 0, 0);

            auto u = float(i) / (width - 1);
            auto v = float(height - 1 - j) / (height - 1);
            std::cerr << "x: " << i << " y: " << j << '\n' << "u: " << u << " v: " << v << '\n';
            Ray r(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            if (std::isnan(r.direction.x) ) throw -1;
            pixel_color += ray_color(r, scene, 0, this->max_reflections);
            
            // std::cerr << pixel_color / samples_per_pixel << ' ' << pixel_color <<' ' << samples_per_pixel << '\n';
            if (!col_in_range(pixel_color / samples_per_pixel)) throw -1;

            row[i] = (pixel_color / samples_per_pixel).gamma(inv_g);
        }

        img[j] = row;
    }
    // bar.mark_as_completed();
    std::cout << "Done\n";
    return img;
}
