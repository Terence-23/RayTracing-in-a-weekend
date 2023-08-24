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
    inline std::string to_json(){
        return "{\"x\":" + std::to_string(x) + ",\"y\":" + std::to_string(y) + ",\"z\":" + std::to_string(z) + "}";
    }

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
    static inline vec3 from_json(std::string json){
        
        // std::cout << "Vec3: " << json << '\n';
        json = remove_whitespace(json);
        size_t l = 0;
        if(json[l] != '{'){
            l = json.find('{');
        }
        size_t r = find_closing(json, l, json.size());
        
        float x = -1, y = -1, z = -1;

        // std::cout << "l: " << l << " r: " << r << '\n';
        for(size_t i = l+1; i < r; ++i){
            if (json[i] == ':'){
                // std::cerr << "Colon at: " << i << '\n';
                // std::cout << "Key: " << json.substr(l+1, i - l - 1) << '\n';
                if (json.substr(l+1, i - l - 1) == "\"x\""){

                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    size_t c = json.find_first_of(",}", ++i);
                    // std::cout << c << ' ' << json[c] << '\n';
                    // std::cerr << json.substr(i, c - i) << '\n';
                    x = atof(json.substr(i, c - i).c_str());
                    // std::cout << "Found x: " << x << '\n';

                } else if (json.substr(l+1, i - l - 1) == "\"y\""){

                    // std::cerr << "List begin: " << json[i+1] << '\n';
                    size_t c = json.find_first_of(",}", ++i);
                    // std::cout << c << ' ' << json[c] << '\n';
                    // std::cerr << json.substr(i, c - i) << '\n';
                    y = atof(json.substr(i, c - i).c_str());
                    // std::cout << "Found y: " << y << '\n';

                } else if (json.substr(l+1, i - l - 1) == "\"z\""){

                    // std::cout << "List begin: " << json[i+1] << '\n';
                    // std::cout << "Z" << std::endl;
                    size_t c = json.find_first_of(",}", ++i);
                    // std::cout << c << ' ' << json[c] << std::endl;
                    // std::cerr << json.substr(i, c - i) << '\n';
                    z = atof(json.substr(i, c - i).c_str());
                    // std::cout << "Found z: " << z << '\n';

                }
            }
            else if (json[i] == ','){
                l = i;
                // std:: cerr << "New l: " << l << '\n';
            }
        }
        
        return vec3(x, y, z);
    }
};

inline vec3 operator * (double t, vec3 v) {
    return v*t;
}

inline RGB_float operator * (RGB_float col, vec3 v) {
    return RGB_float(col.R * v.x, col.G * v.y, col.B * v.z);
}