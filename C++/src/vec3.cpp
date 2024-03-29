#include "vec3.h"

vec3 &vec3::operator/=(const double t)
{
    return *this *= 1/t;
}
vec3& vec3::operator*=(double t){
    x *= t;
    y *= t;
    z *= t;
    return *this;
}

vec3 vec3::operator -() const{
    return vec3(-x, -y, -z);
}

double vec3::length() const
{
    return std::sqrt(this->length2());
}

double vec3::length2() const
{
    return x*x + y*y + z*z;
}

vec3 vec3::random_in_unit_sphere() {
    while (true) {
        auto p = vec3::random(-1,1);
        if (p.length2() >= 1) continue;
        return p;
    }
}

vec3 vec3::random_unit_vec()
{
    return vec3::random_in_unit_sphere().unit_vector();
}

vec3::vec3() : x(0), y(0), z(0)
{
}

vec3::vec3(float x, float y, float z) : x(x), y(y), z(z)
{
}
