#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>
/*
    class to flatten vector<string> into a stream of char
*/

class Flatten {
  class Iterator {
  public:
    using iterator_category = std::bidirectional_iterator_tag;
    using value_type = char;
    using reference = const value_type &;
    using pointer_type = const value_type *;
    using difference_type = std::ptrdiff_t;

    Iterator() = default;

    Iterator &operator++() {
      if (outer_ == last_)
        return *this;
      ++inner_;
      while (outer_ != last_ && inner_ == outer_->end()) {
        ++outer_;
        if (outer_ != last_) {
          inner_ = outer_->begin();
        }
      }
      return *this;
    }

    Iterator &operator--() {
      if (outer_ == last_) {
        --outer_;
        while (outer_->empty()) {
          --outer_;
        }
        inner_ = outer_->end();
      }

      while (inner_ == outer_->begin()) {
        --outer_;
        while (outer_->empty()) {
          --outer_;
        }
        inner_ = outer_->end();
      }

      --inner_;
      return *this;
    }

    Iterator operator++(int) {
      Iterator old = *this;
      ++(*this);
      return old;
    }

    Iterator operator--(int) {
      Iterator old = *this;
      --(*this);
      return old;
    }

    reference operator*() const { return *inner_; }

    friend bool operator==(const Iterator &lhs, const Iterator &rhs) {
      if (lhs.outer_ == lhs.last_ && rhs.outer_ == rhs.last_)
        return true;

      return lhs.outer_ == rhs.outer_ && lhs.inner_ == rhs.inner_;
    }

    friend bool operator!=(const Iterator &lhs, const Iterator &rhs) {
      return not(lhs == rhs);
    }

  private:
    using inner_t = std::string::const_iterator;
    using outer_t = std::vector<std::string>::const_iterator;

    outer_t last_{};
    outer_t outer_{};
    inner_t inner_{};

    Iterator(outer_t first, outer_t last)
        : last_(last),
          outer_(std::find_if_not(first, last,
                                  [](const auto &s) { return s.empty(); })),
          inner_(outer_ == last_ ? inner_t{} : outer_->begin()) {}

    friend class Flatten;
  };

public:
  using iterator = Iterator;
  using const_iterator = Iterator;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  explicit Flatten(std::vector<std::string> v) : flatten_(std::move(v)) {}

  iterator begin() { return {flatten_.begin(), flatten_.end()}; }

  iterator end() { return {flatten_.end(), flatten_.end()}; }

  const_iterator begin() const { return {flatten_.begin(), flatten_.end()}; }

  const_iterator end() const { return {flatten_.end(), flatten_.end()}; }

  const_iterator cbegin() const { return {flatten_.begin(), flatten_.end()}; }

  const_iterator cend() const { return {flatten_.end(), flatten_.end()}; }

  reverse_iterator rbegin() { return reverse_iterator{end()}; }

  reverse_iterator rend() { return reverse_iterator{begin()}; }

  const_reverse_iterator crbegin() const {
    return const_reverse_iterator{end()};
  }

  const_reverse_iterator crend() const {
    return const_reverse_iterator{begin()};
  }

private:
  std::vector<std::string> flatten_;
};

int main() {
  std::vector<std::string> v{"abc", "", "def"};
  Flatten f{v};
  for (auto it = f.begin(); it != f.end(); it++) {
    std::cout << *it << "\n";
  }
  return 0;
}
