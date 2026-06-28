#include <fmt/format.h>

class MyClass {
public:
  explicit MyClass(int i, int j, int k) : a_(i), b_(j), c_(k) {
    fmt::print("constructed myclass with {} {} {}\n", a_, b_, c_);
  }
  
  ~MyClass() {}
private:
  int a_;
  int b_;
  int c_;
};

int main() {
  MyClass mc{1, 2, 3};
  return 0;
}