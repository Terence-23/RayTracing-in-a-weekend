#include "tests.h"


using color = RGB_float;

void write_test()
{
    const int image_width = 256;
    const int image_height = 256;

    // Render
    std::vector<std::vector<RGB_int>> colors_int(image_height);
    std::vector<std::vector<RGB_float>> colors_float(image_height);

    // std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int j = 0; j < image_height; ++j)
    {

        std::vector<RGB_float> float_row(image_width);
        std::vector<RGB_int> int_row(image_width);

        for (int i = 0; i < image_width; ++i)
        {
            RGB_float col(double(i) / (image_width - 1), double(j) / (image_height - 1), 0.25);
            RGB_int int_col(col);
            float_row[i] = col;
            int_row[i] = int_col;
            // std::cout << static_cast<int>(255.999 * double(j) / (image_height-1) )<< " " << col.G << " " << int_col.G << '\n';
        }
        colors_int[j] = int_row;
        colors_float[j] = float_row;
    }
    write_ppm("float_cols.ppm", colors_float);
    write_ppm("int_cols.ppm", colors_int);
}

RGB_float ray_colorV(const Ray &r, const Scene& _scene)
{
    vec3 unit_direction = r.direction.unit_vector();
    auto t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

void viewport_test()
{
    f32 aspect_ratio = 3.0 / 2;
    int width = 600;
    Viewport viewport(width, aspect_ratio);
    auto img = viewport.Render(ray_colorV, Scene());
    write_ppm("viewport_test.ppm", img);
}

RGB_float ray_colorS(const Ray &r, const Scene& _scene)
{

    if (Sphere(vec3(0, 0, -1), 0.5).collide(r))
        return color(1, 0, 0);

    vec3 unit_direction = r.direction.unit_vector();
    auto t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

void sphere_test()
{
    f32 aspect_ratio = 3.0 / 2;
    int width = 600;
    Viewport viewport(width, aspect_ratio);
    auto img = viewport.Render(ray_colorS, Scene());
    write_ppm("sphere_test.ppm", img);
}

RGB_float ray_colorSc(const Ray &r, const Scene& scene)
{
    float mint = 0, maxt = 1000;
    std::vector<Hit> hits;
    for(auto s: scene.spheres){
        Hit hit = s.collisionNormal(r, mint, maxt);
        if(hit.isHit()){
            hits.push_back(hit);
        }
    }

    if (!hits.empty()){
        auto normal = std::min_element(
            hits.begin(), 
            hits.end(), 
            [](const Hit& l, const Hit& r){return l.t < r.t;}
            )->normal;
        return RGB_float(normal.x + 1, normal.y + 1, normal.z + 1) * 0.5;
    }
    auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

void scene_test()
{
    f32 aspect_ratio = 3.0 / 2;
    int width = 900;
    Scene scene;
    std::vector<Sphere> spheres = {
        Sphere(vec3(-0.5, 0, -1), 0.5), 
        Sphere(vec3(0.5, 0, -1), 0.5), 
        Sphere(vec3(0, 0, -2), 1)};
    scene.spheres = spheres;
    Viewport viewport(width, aspect_ratio);
    auto img = viewport.Render(ray_colorSc, scene);
    write_ppm("scene_test.ppm", img);
}

RGB_float ray_colorSN(const Ray &r, const Scene& _scene)
{
    
    vec3 normal = Sphere(vec3(0, 0, -1), 0.5).collisionNormal(r, 0, 1000).normal;
    if (normal != vec3(0, 0, 0))
        return RGB_float(normal.x + 1, normal.y + 1, normal.z + 1) * 0.5;

    auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

RGB_float ray_colorN(const Ray &r, const Scene& _scene)
{
    
    vec3 normal = Sphere(vec3(0, 0, 1), 0.5).collisionNormal(r, 0, 1000).normal;
    if (normal != vec3(0, 0, 0))
        return RGB_float(normal.x + 1, normal.y + 1, normal.z + 1) * 0.5;

    auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

void sphere_normal_test()
{
    f32 aspect_ratio = 3.0 / 2;
    int width = 600;
    Viewport viewport(width, aspect_ratio);
    auto img = viewport.Render(ray_colorSN, Scene());
    write_ppm("sphere_normal_test.ppm", img);
    
    img = viewport.Render(ray_colorN, Scene());
    
    write_ppm("normal_test.ppm", img);
}


void run_tests()
{

    std::vector<void (*)()> funcs = {write_test, viewport_test, sphere_test, sphere_normal_test, scene_test};
    for (const auto f : funcs)
    {
        f();
    }
}