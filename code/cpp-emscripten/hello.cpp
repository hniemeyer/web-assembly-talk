#include <utility>
#include <algorithm>
#include <vector>
#include <iostream>

int main() {
    int number = 1;
    std::cout << "How many fibonacci numbers do you like to see?\n";
    std::cin >> number;
    std::vector<long long int> v;
    auto fib = [first = 0ll, second = 1ll]() mutable {first = std::exchange(second, first + second); return first;};
    std::generate_n(std::back_inserter(v), number, fib);
    for (const auto elem : v) std::cout << elem << '\n';
    return 0;
}
