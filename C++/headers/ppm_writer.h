#ifndef __ppm_writer
#define __ppm_writer

#include"RGB.h"
#include"external_includes.h"

void write_ppm(std::vector<std::vector<RGB_int> > vec);
void write_ppm(std::vector<std::vector<RGB_float> > vec);
void write_test();

#endif

