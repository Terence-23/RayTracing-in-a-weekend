#include "hit.h"

bool Hit::isHit() const {
    return *this != NO_HIT;
}