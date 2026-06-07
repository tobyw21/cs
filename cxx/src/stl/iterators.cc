#include <fmt/ranges.h>
#include <list>
#include <map>
#include <vector>

int main() {
  std::vector<int> vec{1, 2, 3, 4, 5, 6};
  std::list<int> l{1, 2, 3, 4, 5, 6};
  std::map<int, int> m{{1, 1}, {2, 2}, {3, 3}};

  std::vector<int>::iterator vit = vec.begin();
  auto lit = l.begin();
  auto mit = m.begin();

  fmt::print("{}\n", *(vit + 3));

  // Forward/Bidirectional iterator can't do it + 3
  fmt::print("{}\n", *lit);

  auto lit_mid = std::next(l.begin(), std::distance(l.begin(), l.end()) / 2);
  fmt::print("{}\n", *lit_mid);

  fmt::print("{}\n", mit->first);

  fmt::print("vec: {}\n", vec);
  fmt::print("list: {}\n", l);
  fmt::print("map: {}\n", m);
  return 0;
}