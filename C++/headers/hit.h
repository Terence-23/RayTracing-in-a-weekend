#pragma once

#include"vec3.h"
#include"ray.h"


class Hit{
    
public:

    double t, col_mod;
    vec3 normal, point;
    Ray next;

    Hit() = default;
    inline Hit(float t, vec3 normal, vec3 point): t(t), normal(normal), point(point){}
    inline Hit(float t, vec3 normal, vec3 point, float col_mod, Ray next): t(t), normal(normal), point(point), col_mod(col_mod), next(next){}

    bool isHit() const ;
    inline bool operator!=(Hit oth) const { return 
        t != oth.t || 
        normal != oth.normal || 
        point != oth.point || 
        next != oth.next || 
        col_mod != oth.col_mod;
        }; 
    inline bool operator==(Hit o) const {return 
        t == o.t && 
        normal == o.normal && 
        point == o.point && 
        next == o.next && 
        col_mod == o.col_mod;}

};
const Hit NO_HIT = Hit(10000, vec3(0,0,0), vec3(0,0,0), 1, Ray(vec3(0,0,0), vec3(0,0,0)));