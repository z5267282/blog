# Overview

This is a markdown parser for Markdown test, written in Rust.

# Supported Markdown Language Features

Not all language features are supported.  
The full list of features was taken from [markdownguide](https://www.markdownguide.org/basic-syntax/).

## Supported - ✅

### Headings

These must start with leading `'#'` characters followed by one spacebar `' '`.  
There must also be a blank line before and after a heading.

```txt

# Heading

```

### Paragraphs and Line Breaks

A blank line is needed to separate paragraphs.  
Two lines forces a newline.

```txt
Paragraph 1 sentence 1.
Paragraph 1 sentence 2.

Paragraph 2.
```

### Ordered Lists

These must start with a number and then a `'.'`.

```txt
1. one
2. two
```

### Unordered Lists

These must start with `'- '`.

```txt
- apple
- orange
```

### Code Blocks

If a language is provided it must be directly after the ` "```" `.

## Unsupported - ❌

### Priority To Implement

- Links

### Backlog

- Bold Text
- Italic Text
- Blockquotes
- Inline Code Bacticks
- Horizontal Rules
- Images
- HTML

# Assumptions

It is assumed that the Markdown is correctly formatted.
