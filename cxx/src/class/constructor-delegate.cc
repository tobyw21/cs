#include <fmt/format.h>

class MyClass {
public:
  explicit MyClass(int i, int j, int k) : a_(i), b_(j), c_(k) {
    fmt::print("constructed myclass with {} {} {}\n", a_, b_, c_);
  }

  explicit MyClass() : MyClass(1, 2, 3) {
    fmt::print("This is called after delegating constructor\n");
  }

  ~MyClass() {}

private:
  int a_;
  int b_;
  int c_;
};

int main() {
    MyClass mc1 {4, 2, 0};
    MyClass mc2;
    return 0;
}