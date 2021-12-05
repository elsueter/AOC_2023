#include <iostream>
#include <vector>
#include <fstream>

std::vector<int> readFile(){

    std::vector<int> input;
    input.reserve(2000);
    
    std::string file("../input.txt");

    std::ifstream inputFile(file);

    for (std::string line; std::getline(inputFile, line); ) 
    {
        input.push_back(std::stoi(line));
    }
    
    inputFile.close();

    return input;
}

int part1(std::vector<int> input){
    int last = 0;
    int counter = -1;
    for(int i = 0; i < input.size(); i++){
        int next = input[i];
        if(next > last){
            counter++;
        }
        last = next;
    }
    return counter;
}

int part2(std::vector<int> input){
    int last = 0;
    int counter = -1;

    for(int i = 0; i < input.size()-2; i++){
        int next = input[i] + input[i+1] + input[i+2];

        if(next > last){
            counter++;
        }
        last = next;
    }
    return counter;
}

int main(){
    clock_t start, end;

    start = clock();
    
    std::vector<int> input = readFile();

    int pt1 = part1(input);
    int pt2 = part2(input);

    end = clock();

    std::cout<<"Part 1 result: "<<pt1<<std::endl;
    std::cout<<"Part 2 result: "<<pt2<<std::endl;
    std::cout<<"Took: "<<(((double) (end - start)) * 1000000000) / CLOCKS_PER_SEC<<"ns"<<std::endl;

    return 0;
}