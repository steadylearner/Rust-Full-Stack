// The namespace is similar to mod in Rust or package in Golang?
// https://www.learncpp.com/cpp-tutorial/user-defined-namespaces/

#include <iostream>
 
namespace first
{
    // This doSomething() belongs to namespace first
    int doSomething(int x, int y)
    {
        return x + y;
    }
}
 
namespace second
{
    // This doSomething() belongs to namespace second
    int doSomething(int x, int y)
    {
        return x - y;
    }
}
 
int main()
{
    std::cout << second::doSomething(4, 3) << '\n'; // use the doSomething() that exists in second. Should be 1 in cout.
    return 0;
}




