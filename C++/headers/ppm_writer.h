#ifndef __ppm_writer
#define __ppm_writer

#include"RGB.h"
// #include"external_includes.h"
#include <vector>
#include <string>
#include <iostream>
#include <fstream>

void write_ppm(std::string filename, const std::vector<std::vector<RGB_int>>& vec);
void write_ppm(std::string filename, const std::vector<std::vector<RGB_float>>& vec);

void write_ppm(std::ostream &stream, const std::vector<std::vector<RGB_float>>& vec);
void write_ppm(std::ostream &stream, const std::vector<std::vector<RGB_int>>& vec);

#endif

