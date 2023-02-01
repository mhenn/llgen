#include <iostream>


class C0{

    public:
       bool mounted = false;
       bool retrieved_cap = false;
       bool got_base = false;
       bool delivered = false;

    void log(int points){
        std::cout <<  points  << std::endl;
    }

    void retrieve(){
        if(retrieved_cap){
            log(-1);
            return;
        }
        retrieved_cap = true;
        log(1);
    }

    void getBase(){

        if(got_base){
            log(-1);
            return;
        }
        got_base = true;
        log(1);
    }

    void mount(){

        if(mounted){
            log(-10);
            return;
        }
        if(got_base && retrieved_cap){
            log(10);
            mounted = true;
            return;
        }
        log(-1);
    }

    void deliver(){
        if(mounted)
            log(20);
        else
            log(-20);
    }
};
