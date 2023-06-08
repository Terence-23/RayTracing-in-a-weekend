#ifndef __RGB
#define __RGB

#include"external_includes.h"

class RGB_float{
    public:
    double R; double G; double B;
    RGB_float();
    RGB_float(double r, double g, double b);

};

class RGB_int{
    public:
    int R;
    int G;
    int B;
    RGB_int();
    RGB_int(int r, int g, int b);
    RGB_int(RGB_float);
    friend std::ostream & operator <<( std::ostream & os, const RGB_int & col ){    
        os << col.R << ' ' << col.G << ' ' << col.B;
        return os;
    }

};

#endif