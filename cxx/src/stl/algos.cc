#include <algorithm>
#include <fmt/ranges.h>
#include <numeric>
#include <vector>

int main() {

  std::vector<int> v{1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                     11, 12, 13, 14, 15, 16, 17, 18, 19, 20};

  // accumulate
  int sum = std::accumulate(v.begin(), v.end(), 0);
  fmt::print("sum: {}\n", sum);

  int product =
      std::accumulate(v.begin() + 10, v.end(), 1, std::multiplies<int>());
  fmt::print("product: {}\n", product);


  std::string s = "hello world";
  auto to_upper = [](int c) {
    return static_cast<char>(std::toupper(static_cast<unsigned char>(c)));
  };

  // std::string upper_string = std::string(s.size(), '\0');
  // std::transform(s.begin(), s.end(), upper_string.begin(), to_upper);

  std::string upper_string;
  std::transform(s.begin(), s.end(), std::back_inserter(upper_string),
                 to_upper);

  fmt::print("{}\n", upper_string);
  return 0;
}