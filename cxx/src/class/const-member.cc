#include <fmt/format.h>

class Person {
public:
    explicit Person(const std::string& name, int age) : name_(name), age_(age) {}
    
    int get_age() const {
        return age_;
    }

    void set_name(const std::string& n) {
        name_ = n;
    }

    const std::string& get_name() const {
        return name_;
    }
private:
    std::string name_;
    int age_;
};

int main() {
    Person p1{ "Jack", 20 };
    p1.set_name("Blah");

    fmt::print("{}\n", p1.get_name());

    const Person p2{ "smith", 20 };
    // p2.set_name("ahhhhh"); // this will not compile
    fmt::print("{}\n", p2.get_age());
    return 0;
}