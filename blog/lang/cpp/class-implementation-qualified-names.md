# Class-Name Qualification

In C++ when you implement a class in a `.cpp` file you must qualify the class name.

```cpp

// in .h
struct X {
  void f(T);
};

// in .cpp
void X::f(T t = count) { }
```

The only other alternative to qualifying the class name is to write the implementation in the header file.

# Sources

1. [timsong-cpp](https://timsong-cpp.github.io/cppwp/n4659/class.mem#class.mfct-4)
