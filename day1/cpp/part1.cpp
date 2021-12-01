#include <iostream>
#include <string>
#include <fstream>
#include <sstream>

int main(){
    
    std::ifstream file("../input.txt");
    std::string line;

    int last = 0;
    int counter = -1;

    while(std::getline(file, line)){
        std::istringstream strStream(line);

        int next;

        while(strStream >> next){
            if(next > last){
                counter++;
            }
            last = next;
        }
    }

    std::cout<<counter<<std::endl;

    return 0;
}