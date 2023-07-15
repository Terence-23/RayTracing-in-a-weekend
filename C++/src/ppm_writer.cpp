#include "../headers/ppm_writer.h"

void write_ppm(std::string filename, const std::vector<std::vector<RGB_float>>& vec)
{
    //vec[y][x]
    std::ofstream file(filename, std::ios_base::binary);

    write_ppm(file, vec);
    file.close();
}

void write_ppm(std::ostream &stream, const std::vector<std::vector<RGB_float>>& vec){

    u_int64_t imsize[] = {vec[0].size(), vec.size()};
    stream << "P3\n" << imsize[0] << ' ' << imsize[1] << "\n255\n";

    for(auto v: vec){

        for(auto col:v){
            auto int_col = RGB_int(col);
            // std::cerr << col << " " << int_col << '\n';
            stream << int_col << "  ";
        }
        stream << '\n';
    }
}

void write_ppm(std::string filename, const std::vector<std::vector<RGB_int>>& vec)
{
    std::ofstream file(filename, std::ios_base::binary);
    write_ppm(file, vec);
    file.close();
}

void write_ppm(std::ostream &stream, const std::vector<std::vector<RGB_int>>& vec){

    u_int64_t imsize[] = {vec[0].size(), vec.size()};
    stream << "P3\n" << imsize[0] << ' ' << imsize[1] << "\n255\n";

    for(auto v: vec){

        for(auto col:v){
            stream << col << "  ";
        }
        stream << '\n';
    }

}
