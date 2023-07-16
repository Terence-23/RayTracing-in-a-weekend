#pragma once

// #include "external_includes.h"
#include <iostream>
#include <cmath>
#include <defines.h>

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
};

inline vec3 operator * (double t, vec3 v) {
    return v*t;
}