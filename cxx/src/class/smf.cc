#include <iostream>

class Person {
public:
  // Constructor
  Person(int age, const std::string &name, char sex)
      : age_(age), name_(name), sex_(sex) {
    data_ = new int[10]{};
  }

  // Copy constructor
  Person(const Person &p) {
    age_ = p.age_;
    name_ = p.name_;
    sex_ = p.sex_;
    data_ = new int[10]{};
    for (int i = 0; i < 10; i++) {
      data_[i] = p.data_[i];
    }
  }

  // Copy assignment
  Person &operator=(const Person &p) {

    if (this != &p) {
      delete[] data_;
      age_ = p.age_;
      name_ = p.name_;
      sex_ = p.sex_;
      data_ = new int[10]{};
      for (int i = 0; i < 10; i++) {
        data_[i] = p.data_[i];
      }
    }
    return *this;
  }

  // Move constructor
  Person(const Person &&p) = delete;

  // Move assignment
  Person &&operator=(const Person &&p) = delete;

  // Destructor
  ~Person() { delete[] data_; }

  void add_data(int idx, int i) {
    if (idx < 10)
      data_[idx] = i;
  }

  [[nodiscard]] const int *const get_data() const { return data_; }

  friend std::ostream &operator<<(std::ostream &os, const Person &p) {
    os << p.name_ << " " << p.age_ << " " << p.sex_ << " data: ";

    for (int i = 0; i < 10; i++) {
      os << p.data_[i] << " ";
    }
    return os;
  }

private:
  int age_;
  std::string name_;
  char sex_;
  int *data_;
};

int main() {
  Person p1{20, "Alice", 'F'};
  Person p2 = p1;
  p1.add_data(3, 9);
  p2 = p1;

  Person p3{25, "Bob", 'M'};
  p3.add_data(5, 8);
  p2 = p3;

  std::cout << p1 << "\n";
  std::cout << p2 << "\n";
  std::cout << p3 << "\n";
  return 0;
}