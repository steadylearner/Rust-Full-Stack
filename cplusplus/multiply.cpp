#include <iostream>

using namespace std;

int getValueFromUser() {
    std::cout << "Enter an integer: ";
    int input{};
    std::cin >> input;  
 
    return input;
}

void printDouble(int value) {
    std::cout << value << " doubled is: " << value * 2 << '\n';
}

int main() {
    // cout << "Enter an integer: ";

    // int integer{ };
    // cin >> integer;

    // int twoFold = integer * 2;

    // cout << "Double that number is: " << twoFold << "\n";
    // cout << "Double that number is: " << integer * 2 << "\n";

    printDouble(getValueFromUser());

    return 0;
}


