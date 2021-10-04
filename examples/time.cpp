#include <iostream>
#include <math.h>

int main() {
    long long int t1,t2,t3,t4,t5;

    t1 = 5000000000;
    t2 = 6000000000;
    t3 = 3000000000;
    t4 = 2000000000;
    t5 = 10000000000;

    long double res = t1 + t2 * t3 / t4 - t5;

    std::cout << "Size: " << sizeof(res) * 8 << '\n';
    std::cout << "Max Long : " << LONG_MAX << '\n';
    std::cout << "Max Long Long : " << LLONG_MAX << '\n';

    std::cout << "Result" << res;

}