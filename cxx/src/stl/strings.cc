#include <fmt/ranges.h>
#include <string_view>

// string_view, similar to Rust &str
void print_string(const std::string_view &s) { fmt::print("string: {}\n", s); }

int main() {
  print_string(std::string{"this is a string_view from std::string"});
  print_string("this is a string_view from cstring");

  return 0;
}