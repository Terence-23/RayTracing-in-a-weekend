#include"headers/ppm_writer.h"
#include"headers/vec3.h"
#include"headers/external_includes.h"
#include"headers/ray.h"

int main(){
    // write_test();
    vec3 origin(1, 20, 0.5), direction(1,2,0);
    Ray ray(origin, direction);

    std::cout << ray.at(0) << '\n';
    std::cout << ray.at(-2) << '\n';
    return 0;
}