#include <iostream>

using namespace std;

int main() {
    cout << "What is your portfoilo website? "; // www.steadlyearner.com

    string portfolioWebsite{ }; // let mut (String) website?
    cin >> portfolioWebsite;

    std::cout << "Your portfolio website is " << portfolioWebsite << ".\n";
    return 0;
}