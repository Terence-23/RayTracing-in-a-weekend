#pragma once

#include"vec3.h"
#include"ray.h"



class Hit{
    
public:

    double t;
    vec3 normal, point;

    Hit() = default;
    inline Hit(float t, vec3 normal, vec3 point): t(t), normal(normal), point(point){}

    bool isHit() const ;
    inline bool operator!=(Hit oth) const { return t != oth.t || normal != oth.normal || point != oth.point;}; 
    inline bool operator==(Hit o) const {return t == o.t && normal == o.normal && point == o.point;}

};
const Hit NO_HIT = Hit(10000, vec3(0,0,0), vec3(0,0,0));