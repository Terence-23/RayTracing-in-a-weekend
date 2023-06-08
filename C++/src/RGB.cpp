#include"../headers/RGB.h"

RGB_int::RGB_int():R(0), B(0), G(0){
}

RGB_int::RGB_int(int r, int g, int b)
{
    this->R = r;
    this->G = G;
    this->B = b;
}
// std::ostream & RGB_int::operator <<( std::ostream & os, const RGB_int & col )
// {
// }

RGB_int::RGB_int(RGB_float rgb ){
    this->R =static_cast<int>(255.999 * rgb.R); 
    this->G = static_cast<int>(255.999 * rgb.G); 
    this->B = static_cast<int>(255.999 * rgb.B);
}


RGB_float::RGB_float():R(0), B(0), G(0){
}

RGB_float::RGB_float(double r, double g, double b)
{
    this->R = r;
    this->G = g;
    this->B = b;
}

// RGB_float::RGB_int(){
//     return RGB_int(static_cast<int>(255.999 * this->R), static_cast<int>(255.999 * this->G), static_cast<int>(255.999 * this->B))
// }