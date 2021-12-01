#include "common.h"

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
    timer clock;
    clock.start();

    std::vector<int> input = readFile();

    clock.stop();
    clock.start();

    int pt1 = part1(input);
    int pt2 = part2(input);

    clock.stop();

    std::cout<<"Part 1 result: "<<pt1<<std::endl;
    std::cout<<"Part 2 result: "<<pt2<<std::endl;

    return 0;
}