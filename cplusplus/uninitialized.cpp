// #include <iostream>

// using namespace std;

// int main() {
//     // warning: ‘x’ is used uninitialized in this function
//     int x;

//     cout << x;

//     return 0;
// }

#include <iostream>
 
void doNothing(int&) { // Don't worry about what & is for now, we're just using it to trick the compiler into thinking variable x is used

}
 
int main() {
    // define an integer variable named x
    int x; // this variable is uninitialized
 
    doNothing(x); // make the compiler think we're assigning a value to this variable
 
    // print the value of x to the screen (who knows what we'll get, because x is uninitialized)
    std::cout << x;
 
    return 0;
}