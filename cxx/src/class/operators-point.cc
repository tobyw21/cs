#include <iostream>

class Point {
public:
    Point(int x, int y) : x_(x), y_(y) {}

    friend std::ostream& operator<<(std::ostream& os, const Point& p) {
        os << "(" << p.x_ << "," << p.y_ << ")";
        return os;
    }
private:
    int x_;
    int y_;
};

int main() {
    Point p{1,1};
    std::cout << p << "\n";
    return 0;
}