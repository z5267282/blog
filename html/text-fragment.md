# Overview

Text fragments offer a way to direct to a specific spot in an HTML file.  
The documentation from MDM is [here](https://developer.mozilla.org/en-US/docs/Web/URI/Fragment/Text_fragments#browser_compatibility).  
Anchor `<a>` tags in their `href` can redirect to an element's id using a hash like so:

```html
<a href="#os.environ">hello</a>
```

taken from [pydocs](https://docs.python.org/3/library/os.html#os.environ).  
The fragment is captured in the url like so:

```
https://docs.python.org/3/library/os.html#os.environ
```

.

# Known Issues

[Some browsers](https://meta.stackoverflow.com/questions/425878/link-to-a-specific-spot-in-a-stack-overflow-question-answer) support the ability to create fragments based on a selection of text.  
However Firefox does not support this and it is prone to breaking.
