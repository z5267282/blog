# Overview

This blog contains some semantics about Vite I discovered during development of this website.

# Vitest Config

Using vitest, you should use the same `vite.config` and not create a new test config - [link](https://github.com/vitest-dev/vitest/discussions/2053).
Otherwise, the version of React used in testing won't match.  
In this case, I kept getting the error that React wasn't defined which indicated that I was using an outdated version of React where it was mandatory to import React.

# Documentation

This is just a link of some important documentation about assertions.

- [assert text content API](https://vitest.dev/guide/browser/assertion-api.html#tohavetextcontent)
- [expect API](https://vitest.dev/api/expect.html#expect)
