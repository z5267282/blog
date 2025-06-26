# Overview

Constructors have no return type, so how can you prematurely end one if you need to?

# Error

This error will be shown if you try return something inside a constructor.

> error: constructor ... should not return a value [-Wreturn-type]

# Solution

Constructors should throw an exception if they fail.  
Although essentially a constructor is considered to have a return type of `void`.
