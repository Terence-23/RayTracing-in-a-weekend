#ifndef __RGB
#define __RGB

// #include"external_includes.h"

#include <iostream>

class RGB_float{
    public:
    double R; double G; double B;
    RGB_float();
    RGB_float(double r, double g, double b);
    inline RGB_float operator*(double t)const{
        return RGB_float(t*R, t*G, t*B);
    }
    inline RGB_float operator+(RGB_float col)const{
        return RGB_float(R+col.R, G+col.G, B+col.B);
    }

};

inline RGB_float operator*(double t,  RGB_float col){
    return RGB_float(t*col.R, t*col.G, t*col.B);
}

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