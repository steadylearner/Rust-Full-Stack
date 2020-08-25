// $g++ -o main main.cpp add.cpp

#include <iostream>
#include "add.hpp" // Insert contents of add.h at this point.  Note use of double quotes here.

int main()
{
    std::cout << "The sum of 3 and 4 is " << add(3, 4) << '\n';
    return 0;
}