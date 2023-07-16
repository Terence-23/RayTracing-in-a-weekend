#include"headers/ppm_writer.h"
#include"headers/viewport.h"
#include"headers/vec3.h"
#include"headers/external_includes.h"
#include"headers/ray.h"
#include"headers/tests.h"

int main(){
    std::ios::sync_with_stdio(false);
    #ifdef DEBUG
        run_tests();
    #endif

    return 0;
}