#include"tests.h"

#include"../lib/tqdm.cpp/include/tqdm/tqdm.h"

using color = RGB_float;

void write_test(){
    const int image_width = 256;
    const int image_height = 256;

    // Render
    std::vector<std::vector<RGB_int>> colors_int(image_height);
    std::vector<std::vector<RGB_float>>colors_float(image_height);

    // std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int j = 0; j < image_height; ++j) {
        

        std::vector<RGB_float> float_row(image_width);
        std::vector<RGB_int> int_row(image_width);
        
        for (int i = 0; i < image_width; ++i) {
            RGB_float col(double(i) / (image_width-1), double(j) / (image_height-1), 0.25);
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


RGB_float ray_color(const Ray& r) {
    vec3 unit_direction = r.direction.unit_vector();
    auto t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

void viewport_test(){

    f32 aspect_ratio = 3.0/2;
    int width = 600;
    int height = static_cast<int>(width/aspect_ratio);

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    std:: cout<< "\nH: " << height <<" W: " << width << " W/H: " << double(width)/height << "\nAspect ratio: " << aspect_ratio << '\n';

    vec3 origin(0,0,0);
    vec3 horizontal(viewport_width, 0, 0), vertical(0, viewport_height, 0);
    vec3 lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);

    std::vector<std::vector<RGB_float>> img(height);
    for (int j: tqdm::range(height)){
        std::vector<RGB_float> row(width);
        
        for(int i = 0; i < width; ++i){
            auto u = double(i) / (width-1);
            auto v = double(j) / (height-1);
            Ray r(origin, lower_left_corner + horizontal*u + vertical*v - origin);
            row[i] = ray_color(r);
        }

        img[j] = row;
    }
    write_ppm("viewport_test.ppm", img);

}

void run_tests(){

    std::vector<void(*)()> funcs = {write_test, viewport_test};
    for( const auto f: tqdm::tqdm(funcs)){
        
        f();
    }


}