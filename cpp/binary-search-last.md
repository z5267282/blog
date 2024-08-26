The `upper_bound` and `lower_bound` functions give iterators to the first element matching a condition.  
It is possible to change the behaviour so that the last position is instead returned.  
Use reverse iterators in conjunction with the `greater<N>` comparator to do this.

```cpp
#include <algorithm>
#include <cassert>
#include <vector>

auto main(void) -> int {
    /**
        what is the biggest index from [2, 6) st.
        boxes[i] <= 6
        it is 2, boxes[2] = 5
     */
    auto boxes = std::vector<int>{1 , 3 , 5 , 10, 14, 18};
    //                            0   1   2   3   4   5
    // reverse                 e                      b
    //                         -- ->                  <- ++
    //                            18, 14, 10, 5 , 3 , 1
    //                            0   1   2   3   4   5
    //                                        ^ should be this index
    auto j = std::lower_bound(boxes.rbegin(), boxes.rend() - 1 - 1, 6) - boxes.rbegin();
    assert(j == 3);
}
```

