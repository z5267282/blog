Use `string.ascii_lowercase` to easily build a frequency table of lowercase English letters.  
This helps create easy iteration through all cased-characters since:

- Python does not have a `char` type;
- it's otherwise quite annoying to iterate through ASCII values by using `ord()`

```py
from string import ascii_lowercase as lc

freq = { l : 0 for l in lc }
msg = 'hello'

for m in msg:
    freq[m] += 1

print(freq) # { 'e': 1, 'h': 1, 'l': 2, 'o': 1 }
```
