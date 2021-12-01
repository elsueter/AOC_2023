#include <iostream>
#include <chrono>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>

struct timer{
    std::chrono::_V2::system_clock::time_point startClock;
    std::chrono::_V2::system_clock::time_point stopClock;
    void start(){
        startClock = std::chrono::high_resolution_clock::now();
    }
    void stop(){
        stopClock = std::chrono::high_resolution_clock::now();
        auto nsInt = std::chrono::duration_cast<std::chrono::nanoseconds>(stopClock - startClock);
        std::cout<<"Took: "<<nsInt.count()<<"ns"<<std::endl;
    }
};

std::vector<int> readFile(){
    std::ifstream file("../input.txt");
    std::string line;

    std::vector<int> input;

    while(std::getline(file, line)){
        std::istringstream strStream(line);

        int n;

        while(strStream >> n){
            input.push_back(n);
        }
    }
    return input;
}