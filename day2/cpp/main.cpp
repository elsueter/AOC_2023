#include <iostream>
#include <vector>
#include <fstream>
#include <chrono>

struct dirPair{
    std::string dir;
    int dist;
};

dirPair splitString(std::string in){
    char splitChar = ' ';
    std::vector<std::string> parts = {""};

    dirPair output;

    int part = 0;
    for(int i = 0; i < in.size(); i++){
        parts[part] += in[i];
        if(in[i] == splitChar){
            part++;
            parts.push_back("");
        }
    }

    output.dir = parts[0];
    output.dist = std::stoi(parts[1]);

    return output;
}

std::vector<dirPair> readFile(){

    std::vector<dirPair> input;
    input.reserve(2000);
    
    std::string file("../input.txt");

    std::ifstream inputFile(file);

    for (std::string line; std::getline(inputFile, line); ) 
    {
        input.push_back(splitString(line));
    }
    
    inputFile.close();

    return input;
}

int part1(std::vector<dirPair> input){
    int horPos = 0;
    int verPos = 0;

    for(auto &it: input){
        switch(it.dir[0]){
            case 'f':
                horPos += it.dist;
                break;
            case 'u':
                verPos -= it.dist;
                break;
            case 'd':
                verPos += it.dist;
                break;
            default:
                break;
        }
    }

    return horPos*verPos;
}

int part2(std::vector<dirPair> input){
    int horPos = 0;
    int verPos = 0;
    int aim = 0;

    for(auto &it: input){
        switch(it.dir[0]){
            case 'f':
                horPos += it.dist;
                verPos += (aim*it.dist);
                break;
            case 'u':
                aim -= it.dist;
                break;
            case 'd':
                aim += it.dist;
                break;
            default:
                break;
        }
    }

    return horPos*verPos;
}

int main(){
    auto t1 = std::chrono::high_resolution_clock::now();
    
    std::vector<dirPair> input = readFile();

    int pt1 = part1(input);
    int pt2 = part2(input);

    auto t2 = std::chrono::high_resolution_clock::now();
    auto ms_int = std::chrono::duration_cast<std::chrono::nanoseconds>(t2 - t1);

    std::cout<<"Part 1 result: "<<pt1<<std::endl;
    std::cout<<"Part 2 result: "<<pt2<<std::endl;
    std::cout<<"Took: "<<ms_int.count()<<"ns\n";

    return 0;
}