#pragma once

#include"sphere.h"
#include"external_includes.h"


class Scene
{
private:
    /* data */
public:

    std::vector<Sphere> spheres;

    inline Scene(): spheres(){};
    ~Scene() = default;

    inline std::string to_json(){
        std::string sphere_s = "[";
        for( auto i: spheres){
            sphere_s += i.to_json() + ',';
        }
        sphere_s.erase(sphere_s.end() - 1);
        sphere_s += ']';

        return "{\"spheres\":" + sphere_s + "}";
    }
    static inline Scene from_json(std::string json, int l, int r){
        json = remove_whitespace(json);
        if(json[l] != '{'){
            l = json.find('{');
        }
        r = find_closing(json, l, r);
        std::vector<Sphere> spheres(0);
        std::cerr << "l: " << l << "r: " << r << '\n';

        for(size_t i = l+1; i < r; ++i){
            if (json[i] == ':'){
                std::cerr << "Colon at: " << i << '\n';
                std::cerr << "Key: " << json.substr(l+1, i - l - 1) << '\n';
                if(json.substr(l+1, i - l - 1) == "\"spheres\""){
                    std::cerr << "List begin: " << json[i+1] << '\n';
                    auto c = find_closing(json, ++i, r);
                    ++i;
                    std::cerr << json.substr(i, c - i) << '\n';
                    
                    int end;
                    while (i < c){
                        end = find_closing(json, i, c);
                        if (end == i) return Scene();
                        spheres.push_back(Sphere::from_json(json.substr(i, end - i + 1)));
                        if (json[end + 1] != ',' && end+1 < c) return Scene();
                        i = end + 2;
                    }
                    std::cout << '\n';
                }
            }
            else if (json[i] == ','){
                l = i;
                std:: cerr << "New l: " << l << '\n';
            }
            
        }
        Scene scene;
        scene.spheres = spheres;
        return scene;
    }
};




