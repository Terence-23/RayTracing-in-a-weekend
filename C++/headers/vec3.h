#pragma once

// #include "external_includes.h"
#include <iostream>
#include <cmath>
#include "defines.h"
#include "RGB.h"

class vec3{

private:

public:
    float x, y, z; 
    vec3();
    vec3(float x, float y, float z);
    vec3(const vec3& v) = default;

    vec3 operator -()const;
    vec3& operator +=(const vec3& v);
    vec3& operator *=(const double t);
    
    vec3& operator /=(const double t);

    double length()const;
    double length2()const;

    friend std::ostream & operator <<( std::ostream & os, const vec3 & v ){    
        os << v.x << ' ' << v.y << ' ' << v.z;
        return os;
    }
    inline static vec3 random_in_unit_disk() {
        while (true) {
            auto p = vec3(random_double(-1,1), random_double(-1,1), 0);
            if (p.length2() < 1)
                return p;
        }
    }
    inline bool near_zero() const {
        // Return true if the vector is close to zero in all dimensions.
        const auto s = 1e-8;
        return (fabs(x) < s) && (fabs(y) < s) && (fabs(z) < s);
    }

    inline vec3 unit_vector() const{
        return *this / this->length();
    }

    inline vec3 operator+(const vec3 &v) const{
        return vec3(x + v.x, y + v.y, z + v.z);
    }

    inline vec3 operator-(const vec3 &v) const{
        return vec3(x - v.x, y - v.y, z - v.z);
    }

    inline vec3 operator*(double t) const{
        return vec3(t*x, t*y, t*z);;
    }

    inline vec3 operator/( double t) const{
        return *this*(1/t);
    }

    inline double dot(const vec3 &v) const{
        return x * v.x
            + y * v.y
            + z * v.z;
    }

    inline vec3 cross( const vec3 &v) const{
        return vec3(y * v.z - z * v.y,
                    z * v.x - x * v.z,
                    x * v.y - y * v.x);
    }
    inline bool operator!=(const vec3& v)const{
        return x != v.x || y != v.y || z != v.z;
    }
    inline bool operator==(const vec3& v)const{
        return x == v.x && y == v.y && z == v.z;
    }

    inline static vec3 random() {
        return vec3(random_double(), random_double(), random_double());
    }

    inline static vec3 random(double min, double max) {
        return vec3(random_double(min,max), random_double(min,max), random_double(min,max));
    }
    static vec3 random_in_unit_sphere();
    static vec3 random_unit_vec();
    vec3 reflect(const vec3& n) const {
        // reflect vector around a normal N
        return *this - n * this->dot(n) * 2 ;
    }
};

inline vec3 operator * (double t, vec3 v) {
    return v*t;
}

inline RGB_float operator * (RGB_float col, vec3 v) {
    return RGB_float(col.R * v.x, col.G * v.y, col.B * v.z);
}