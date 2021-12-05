#include <iostream>
#include <vector>
#include <fstream>
#include <cmath>
#include <chrono>

typedef std::vector<bool> reading;

void printReading(reading input){
    for(int i = 0; i < input.size(); i++){
        std::cout<<input[i];
    }
    std::cout<<std::endl;
}

int reading2Int(reading input){
    int output = 0;

    for(int i = 0; i < input.size(); i++){
        output += input[((input.size()-1)-i)]*(std::pow(2, i));
    }

    return output;
}

reading string2Reading(std::string input)
{
    reading output;

    for(int i = 0; i < input.size(); i++){
        if(input[i] == '0'){
            output.push_back(false);
        }else{
            output.push_back(true);
        }
    }

    return output;
}

std::vector<reading> readFile()
{
    std::vector<reading> input;
    input.reserve(2000);
    
    std::string file("../input.txt");

    std::ifstream inputFile(file);

    for (std::string line; std::getline(inputFile, line); ) 
    {
        input.push_back(string2Reading(line));
    }
    
    inputFile.close();

    return input;
}

reading getRawGamma(std::vector<reading> input){
    std::vector<int> counter;
    for(int i = 0; i < input[0].size(); i++){
        counter.push_back(0);
    }

    reading rawGamma;

    for(int i = 0; i < input.size(); i++){
        for(int j = 0; j < input[i].size(); j++){
            if(input[i][j]){
                counter[j]++;
            }
        }
    }

    for(int i = 0; i < counter.size(); i++){
        if(counter[i] >= input.size()/2){
            rawGamma.push_back(1);
        }else{
            rawGamma.push_back(0);
        }
    }

    return rawGamma;
}

int part1(reading rawGamma)
{
    uint mask = (1<<rawGamma.size()) - 1;
    uint gamma = reading2Int(rawGamma);
    uint epsilon = mask & ~gamma;

    return gamma*epsilon;
}

int part2(std::vector<reading> input)
{
    std::vector<reading> oxygenReadings = input;
    std::vector<reading> co2Readings = input;

    for(int i = 0; i < oxygenReadings[0].size(); i++){

        int trueCounter = 0;
        int falseCounter = 0;
        for(int j = 0; j < oxygenReadings.size(); j++){
            if(oxygenReadings[j][i]){
                trueCounter++;
            }else{
                falseCounter++;
            }
        }

        bool mostCommon = true;
        if(trueCounter < falseCounter){
            mostCommon = false;
        }

        for(int j = 0; j < oxygenReadings.size(); j++){
            if(oxygenReadings[j][i] != mostCommon){
                oxygenReadings.erase(oxygenReadings.begin()+j);
                j--;
            }
            if(oxygenReadings.size() == 1){
                break;
            }
        }

        if(oxygenReadings.size() == 1){
            break;
        }
    }

    for(int i = 0; i < co2Readings[0].size(); i++){

        int trueCounter = 0;
        int falseCounter = 0;
        for(int j = 0; j < co2Readings.size(); j++){
            if(co2Readings[j][i]){
                trueCounter++;
            }else{
                falseCounter++;
            }
        }

        bool mostCommon = true;
        if(trueCounter < falseCounter){
            mostCommon = false;
        }

        for(int j = 0; j < co2Readings.size(); j++){
            if(co2Readings[j][i] == mostCommon){
                co2Readings.erase(co2Readings.begin()+j);
                j--;
            }
            if(co2Readings.size() == 1){
                break;
            }
        }

        if(co2Readings.size() == 1){
            break;
        }
    }

    return reading2Int(co2Readings[0])*reading2Int(oxygenReadings[0]);
}

int main(){
    auto t1 = std::chrono::high_resolution_clock::now();
    
    std::vector<reading> input = readFile();
    reading rawGamma = getRawGamma(input);

    int pt1 = part1(rawGamma);
    int pt2 = part2(input);

    auto t2 = std::chrono::high_resolution_clock::now();
    auto ms_int = std::chrono::duration_cast<std::chrono::nanoseconds>(t2 - t1);

    std::cout<<"Part 1 result: "<<pt1<<std::endl;
    std::cout<<"Part 2 result: "<<pt2<<std::endl;
    std::cout<<"Took: "<<ms_int.count()<<"ns\n";


    return 0;
}