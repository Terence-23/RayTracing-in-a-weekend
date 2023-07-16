#ifndef __ray
#define __ray

#include"vec3.h"

class Ray{
public: 
    vec3 origin, direction;
    Ray();
    Ray(const vec3 &origin);
    Ray(const vec3 &origin, const vec3& direction);

    vec3 at(double t) const;

    inline bool operator==(Ray o) const {
        return origin == o.origin && direction == o.direction;
    }
    inline bool operator!=(Ray o) const {
        return origin != o.origin || direction != o.direction;
    }
    friend std::ostream & operator <<( std::ostream & os, const Ray & v ){    
        os << v.origin << ' ' << v.direction;
        return os;
    }

};

#endif