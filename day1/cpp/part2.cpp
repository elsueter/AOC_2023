#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>

int main(){
    
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

    int last = 0;
    int counter = -1;

    for(int i = 0; i < input.size()-2; i++){
        int next = input[i] + input[i+1] + input[i+2];

        if(next > last){
            counter++;
        }
        last = next;
    }

    std::cout<<counter<<std::endl;

    return 0;
}