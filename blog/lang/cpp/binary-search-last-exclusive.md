# Overview

When using `upper_bound` and `lower_bound` from `<algorithm>` the end iterator range is not inclusive.

```cpp
#include <algorithm>
#include <cassert>
#include <vector>

auto main(void) -> int {
    auto nums = std::vector<int>{10, 20, 30};
    // essentially find the first element > 25 out of {10, 20}
    // nums.end() - 1 points to the index of 30
    auto idx = std::upper_bound(nums.begin(), nums.end() - 1, 25);
    assert(idx == nums.end() - 1);
}
```
