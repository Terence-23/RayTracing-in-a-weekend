#include "../headers/ppm_writer.h"

void write_ppm(std::string filename, std::vector<std::vector<RGB_float>>& vec)
{

    //vec[y][x]
    u_int32_t imsize[] = {vec[0].size(), vec.size()};
    std::ofstream file(filename, std::ios_base::binary);

    file << "P3\n" << imsize[0] << ' ' << imsize[1] << "\n255\n";

    for(auto v: vec){

        for(auto col:v){
            file << RGB_int(col) << "  ";
        }
        file << '\n';
    }

    file.close();

}

void write_ppm(std::string filename, std::vector<std::vector<RGB_int>>& vec)
{

    //vec[y][x]
    u_int32_t imsize[] = {vec[0].size(), vec.size()};
    std::ofstream file(filename, std::ios_base::binary);

    file << "P3\n" << imsize[0] << ' ' << imsize[1] << "\n255\n";
    // std::cout << "P3\n" << imsize[0] << ' ' << imsize[1] << "\n255\n";

    for(auto v: vec){

        for(auto col:v){
            file << col << "  ";
            // std::cout << col << "  ";
        }
        file << '\n';
        // std::cout << '\n';
    }
    file.close();
}

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