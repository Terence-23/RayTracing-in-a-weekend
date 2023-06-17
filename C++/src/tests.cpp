#include"tests.h"




void write_test(){
    const int image_width = 256;
    const int image_height = 256;

    // Render
    std::vector<std::vector<RGB_int>> colors_int(image_height);
    std::vector<std::vector<RGB_float>>colors_float(image_height);

    std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

    for (int j = 0; j < image_height; ++j) {
        

        std::vector<RGB_float> float_row(image_width);
        std::vector<RGB_int> int_row(image_width);
        
        for (int i = 0; i < image_width; ++i) {
            RGB_float col(double(i) / (image_width-1), double(j) / (image_height-1), 0.25);
            RGB_int int_col(col);
            float_row[i] = col;
            int_row[i] = int_col;
            std::cout << static_cast<int>(255.999 * double(j) / (image_height-1) )<< " " << col.G << " " << int_col.G << '\n';
        }
        colors_int[j] = int_row;
        colors_float[j] = float_row;
    }
    write_ppm("float_cols.ppm", colors_float);
    write_ppm("int_cols.ppm", colors_int);
}

vec3 unit_vector(vec3 v){
    return v/v.length();
}
RGB_float ray_color(const Ray& r) {
    vec3 unit_direction = unit_vector(r.direction);
    auto t = 0.5*(unit_direction.y + 1.0);
    return RGB_float(t, 1-t, 0);
}

void viewport_test(){

    f32 aspect_ratio = 3/2;
    int width = 600;
    int height = static_cast<int>(width/aspect_ratio);

    auto viewport_height = 2.0;
    auto viewport_width = aspect_ratio * viewport_height;
    auto focal_length = 1.0;

    vec3 origin(0,0,0);
    vec3 horizontal(viewport_width, 0, 0), vertical(0, viewport_height, 0);
    vec3 lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);


}

void run_tests(){

    write_test();


}