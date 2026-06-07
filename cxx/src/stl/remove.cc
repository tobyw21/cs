#include <algorithm>
#include <fmt/ranges.h>
#include <vector>

template <typename ForwardIt, typename UnaryPredicate>
ForwardIt my_remove_if(ForwardIt start, ForwardIt end, UnaryPredicate pred) {
  while (start != end && !pred(*start)) {
    start++;
  }

  ForwardIt it = start;
  it++;
  for (; it != end; it++) {
    if (!pred(*it)) {
      *start = std::move(*it);
      start++;
    }
  }

  return start;
}

int main() {
  std::vector<int> v{1,  2,  3,  4,  5,  6,  7,  8,  9,  10,
                     11, 12, 13, 14, 15, 16, 17, 18, 19, 20};

  auto pred = [](int x) { return x % 2 == 0; };
  auto it = my_remove_if(v.begin(), v.end(), pred);
  // remove_if
  // auto it = std::remove_if(v.begin(), v.end(), pred);

  fmt::print("before removal: {}\n", v);
  // erase-remove_if idiom
  v.erase(it, v.end());

  fmt::print("after removal: {}\n", v);
  return 0;
}