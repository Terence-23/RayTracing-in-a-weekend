#ifndef __defines
#define __defines

#define f32 float
#include<cstdlib>
#include<limits>
#include<string>
#include<vector>


// Constants

const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;

// Utility Functions

inline double degrees_to_radians(double degrees) {
    return degrees * pi / 180.0;
}

inline double random_double() {
    // Returns a random real in [0,1).
    return rand() / (RAND_MAX + 1.0);
}

inline double random_double(double min, double max) {
    // Returns a random real in [min,max).
    return min + (max-min)*random_double();
}

inline double clamp(double x, double min, double max) {
    if (x < min) return min;
    if (x > max) return max;
    return x;
}

inline size_t find_closing(std::string str, size_t p, size_t e){
    // p - index of beginning parenthesis 
    // e - end of span 
    char close;
    switch (str[p]){
        case '{':
            close = '}';
            break;
        case '[':
            close = ']';
            break;
        case '(':
            close = ')';
            break;
        default: 
            return p;
    }
    
    e = std::min(e, str.size());
    int p_count = 0, b_count = 0, bk_count = 0;
    std::cerr << "Close: " << close << '\n';

    for (size_t i = p+1; i < e; ++i)
    {
        
        switch (str[i]){
            case '{':
                b_count++;
                // std::cerr << "b_count: "<< b_count << '\n';
                break;
            case '[':
                bk_count++;
                // std::cerr << "bk_count: "<< b_count << '\n';
                break;
            case '(':
                p_count++;
                // std::cerr << "p_count: "<< b_count << '\n';
                break;
            case '}':
                b_count--;
                // std::cerr << "b_count: "<< b_count << '\n';
                break;
            case ']':
                bk_count--;
                // std::cerr << "bk_count: "<< b_count << '\n';
                break;
            case ')':
                p_count--;
                // std::cerr << "p_count: "<< b_count << '\n';
                break;
        }
        if (p_count < 0 || b_count < 0 || bk_count < 0){
            // std::cerr << "b_count: " << b_count << '\n';
            return str[i] == close ? i : p;
        }
    }
    std::cerr << "no find\n";
    return p;
}

inline std::string remove_whitespace(std::string str){
    std::string ret_val = "";
    for( auto i : str){
        if (!isspace(i))ret_val+= i;
    }
    return ret_val;
}

inline std::vector<std::string> split(std::string str, char d){

    std::vector<std::string> ret_val = std::vector<std::string>();
    size_t begin = 0;
    for (size_t i = 0; i < str.size(); ++i){
        if (str[i] == d){
            ret_val.push_back(str.substr(begin, i - begin));
            begin = ++i;
        }
    }
    ret_val.push_back(str.substr(begin));
    return ret_val;
}

#endif