#include "viewport.h"
#include "../lib/indicators/single_include/indicators/indicators.hpp"

Img Viewport::Render(RGB_float (*ray_color)(const Ray &r, const Scene &scene, uint depth, uint max_depth), const Scene &scene)
{

    // std::cout << "\nH: " << height << " W: " << width << " W/H: " << double(width) / height << "\nAspect ratio: " << aspect_ratio << '\n';

    std::vector<std::vector<RGB_float>> img(cam.height);

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
        option::MaxProgress{cam.height}
    };

    for (int j =0; j < cam.height; j ++ )
    {
        std::vector<RGB_float> row(cam.width);
        bar.set_progress(j);

        for (int i = 0; i < cam.width; ++i)
        {
            RGB_float pixel_color = RGB_float(0, 0, 0);
            // set progress bar
            
            for (int s = 0; s < samples_per_pixel; s++)
            {
                Ray r(cam.origin, cam.pixel00_loc + (i + random_double()) * cam.pixel_delta_u + (j+ random_double()) * cam.pixel_delta_v);
                // std::cerr << r.direction << '\n';
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

    std::vector<std::vector<RGB_float>> img(cam.height);

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

    for (int j =0; j < cam.height; j ++ )
    {
        std::vector<RGB_float> row(cam.width);
        // bar.set_progress(j);

        for (int i = 0; i < cam.width; ++i)
        {
            RGB_float pixel_color = RGB_float(0, 0, 0);

            Ray r(cam.origin, cam.pixel00_loc + i * cam.pixel_delta_u + j * cam.pixel_delta_v);
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
