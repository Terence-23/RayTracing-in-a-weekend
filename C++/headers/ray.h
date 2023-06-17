#ifndef __ray
#define __ray

#include"vec3.h"

class Ray{
public: 
    vec3 origin, direction;
    Ray();
    Ray(const vec3 &origin);
    Ray(const vec3 &origin, const vec3& direction);

    vec3 at(double t);


};

#endif