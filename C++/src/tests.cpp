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

RGB_float ray_colorV(const Ray &r, const Scene& _scene, uint _depth, uint _max_depth)
{
    vec3 unit_direction = r.direction.unit_vector();
    auto t = 0.5 * (unit_direction.y + 1.0);
    color pix = (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
    if (pix.R > 1 || pix.G > 1 || pix.B > 1) throw -1;
    return pix;
}

void viewport_test()
{
    f32 aspect_ratio = 3.0 / 2;
    int width = 600;
    Viewport viewport(width, aspect_ratio, 1);
    auto img = viewport.Render(ray_colorV, Scene());
    write_ppm("viewport_test.ppm", img);
}

RGB_float ray_colorS(const Ray &r, const Scene& _scene, uint _depth, uint _max_depth)
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
    Viewport viewport(width, aspect_ratio, 1);
    auto img = viewport.Render(ray_colorS, Scene());
    write_ppm("sphere_test.ppm", img);
}

RGB_float ray_colorSc(const Ray &r, const Scene& scene, uint _depth, uint _max_depth)
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
    uint samples = 100;
    Scene scene;
    std::vector<Sphere> spheres = {
        Sphere(vec3(-0.5, 0, -1), 0.5), 
        Sphere(vec3(0.5, 0, -1), 0.5), 
        Sphere(vec3(0, 0, -2), 1)};
    scene.spheres = spheres;
    Viewport viewport(width, aspect_ratio, samples);
    auto img = viewport.Render(ray_colorSc, scene);
    write_ppm("scene_test.ppm", img);
}

RGB_float ray_colorSN(const Ray &r, const Scene& _scene, uint _depth, uint _max_depth)
{
    
    vec3 normal = Sphere(vec3(0, 0, -1), 0.5).collisionNormal(r, 0, 1000).normal;
    if (normal != vec3(0, 0, 0))
        return RGB_float(normal.x + 1, normal.y + 1, normal.z + 1) * 0.5;

    auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

RGB_float ray_colorN(const Ray &r, const Scene& _scene, uint _depth, uint _max_depth)
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
    Viewport viewport(width, aspect_ratio, 1);
    auto img = viewport.Render(ray_colorSN, Scene());
    write_ppm("sphere_normal_test.ppm", img);
    
    img = viewport.Render(ray_colorN, Scene());
    
    write_ppm("normal_test.ppm", img);
}

RGB_float ray_colorD(const Ray &r, const Scene& scene, uint depth, uint max_depth)
{
    // std::cout << r << '\n';
    if (depth >= max_depth) {
        // auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
        // return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
        return RGB_float(0,0,0);
    }
    float mint = 0.001, maxt = 1000;
    Hit min_hit = NO_HIT;
    for(auto s: scene.spheres){
        Hit hit = s.collisionNormal(r, mint, maxt);
        if(hit.isHit() && (!min_hit.isHit() || hit.t < min_hit.t)){
            min_hit = hit;
        }
    }

    if (min_hit.isHit()){
        
        RGB_float col = ray_colorD(min_hit.next, scene, depth+1, max_depth) * min_hit.col_mod;
        
        return col;
    }
    auto t = 0.5 * (r.direction.unit_vector().y + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}


void diffuse_test()
{
    // uint height = 600;
    f32 aspect_ratio = 3.0 / 2;
    int width = 900;
    uint samples = 100;
    Scene scene;
    std::vector<Sphere> spheres = {
        Sphere(vec3(-0.5, 0, -1), 0.5, materials::scatterM, vec3(0.6, 0.6, 0.6)), 
        Sphere(vec3(0.5, 0, -1), 0.5, materials::scatterM, vec3(1, 0.2, 0.2)), 
        Sphere(vec3(0, 0, -2), 1, materials::scatterM, vec3(0.2, 0.2, 0.2))};
    scene.spheres = spheres;
    // Viewport viewport(width, height, samples, 10);
    Viewport viewport(width, aspect_ratio, samples, 10);
    auto img = viewport.Render(ray_colorD, scene);
    write_ppm("diffuse_test.ppm", img);
}
void metal_test()
{
    // uint height = 600;
    f32 aspect_ratio = 3.0 / 2;
    int width = 900;
    uint samples = 100;
    Scene scene;
    std::vector<Sphere> spheres = {
        Sphere(vec3(-0.52, 0, -1), 0.5, materials::scatterM, vec3(0.6, 0.6, 0.6)), 
        Sphere(vec3(0.52, 0, -1), 0.5, materials::scatterM, vec3(1, 0.2, 0.2)), 
        Sphere(vec3(0, 0, -3), 1, materials::metallicM, vec3(1, 1, 1)),
        Sphere(vec3(0, -1000.6, -40), 1000, materials::fuzzy3, vec3(1, 0, 1))};
    scene.spheres = spheres;
    // Viewport viewport(width, height, samples, 10);
    Viewport viewport(width, aspect_ratio, samples, 10);
    auto img = viewport.Render(ray_colorD, scene);
    write_ppm("metal_test.ppm", img);
}
void glass_test()
{
    // uint height = 600;
    f32 aspect_ratio = 3.0 / 2;
    int width = 300;
    uint samples = 100;
    Scene scene;
    std::vector<Sphere> spheres = {
        Sphere(vec3(0/*-0.52*/, 0, -1), 0.5, materials::glass, vec3(1, 1, 1)),
        Sphere(vec3(0/*-0.52*/, 0, -1), 0.35, materials::glassR, vec3(1, 1, 1)), 
        // Sphere(vec3(0.52, 0, -1), 0.5, materials::scatterM, vec3(1, 0.2, 0.2)), 
        // Sphere(vec3(0, 0, -3), 1, materials::metallicM, vec3(1, 1, 1)),
        Sphere(vec3(0, -100.5, -1), 100, materials::scatterM, vec3(1, 0, 1))};
    scene.spheres = spheres;
    // Viewport viewport(width, height, samples, 10);
    Viewport viewport(width, aspect_ratio, samples, 10);
    auto img = viewport.Render(ray_colorD, scene);
    write_ppm("glass_test.ppm", img);
}

class Test{
    public:
    const char* message;
    void (*test_f)();
    Test(const char* message,  void (*test_f)()): message(message), test_f(test_f){}
    void run(){
        std::cout << message << "\n";
        test_f();
    }
};


void run_tests()
{
    std::vector<Test> tests = {
        // Test("Write test", write_test), 
        // Test("Viewport test", viewport_test), 
        // Test("Sphere test", sphere_test), 
        // Test("Sphere normal test", sphere_normal_test), 
        // Test("Scene test", scene_test), 
        // Test("Diffuse material test", diffuse_test),
        // Test("Metal material test", metal_test),
        Test("Dielectric material test", glass_test)
    };
    for (auto t: tests) t.run();
}