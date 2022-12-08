#include <string.h>


class Order{
    public:
    Order(){
        this->base = "";
        this->cap = "";
    }
    Order(std::string b, std::string c){
        this->base = b;
        this->cap = c;
    }

    std::string base;
    std::string cap;

};

Order *orderInstance = new Order();
