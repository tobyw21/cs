#include <fmt/ranges.h>
#include <span>
#include <vector>

// Span, similar to Rust &[T]
template <typename T> void print_container(std::span<T> s) {
  fmt::print("slice: {}\n", s);
}

int main() {

  std::vector<int> v{1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                     11, 12, 13, 14, 15, 16, 17, 18, 19, 20};

  print_container(std::span<const int>{v}.subspan(5, 10));
  return 0;
}