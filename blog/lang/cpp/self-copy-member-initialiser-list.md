# The Problem

For the copy-assignment operator, the `this` object has already been constructed.  
So you do have to check for self copying.  
However, this operator is not a **constructor** so you don't have to worry about setting anything in the member intialiser list.
