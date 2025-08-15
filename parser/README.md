# Overview

This is a markdown parser for Markdown test, written in Rust.  
The full json schema is written [here](./json_schema.md).

This command will create the file `../website/public/blogs.json`.

```sh
cargo run
```

## Options

To add logging, prefix with this argument.

```sh
RUST_LOG=info ./target/debug/parser
```

To turn on pretty printing, add this argument.

```sh
cargo run -- --pretty
```

# Formatting

It is expected there is a blank line to end a particular markdown feature.

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
It is assumed that the lists are correctly enumerated from `[1,n]` for an `n`-sized list.

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

### Tables

Tables must be formatted like so.

```txt
| header 1 | header 2 ... |
| -------- | ------------ |
| content  | goes         |
| here     | ...          |
```

The second row must start with a pipe `|`.  
All content has its leading and trailing whitespace trimmed so the rows above are parsed to the following.

```json
[
  ["content", "goes"],
  ["here", "..."]
]
```

## Frontend Rendered

These are supported if nested inside a paragraph.  
They will be parsed by the frontend as they only involve simple single-line string manipulations.

- Links
- Bold Text, where asterisks are used `** bold text **`
- Inline Code Bacticks

## Unsupported - ❌

- Italic Text
- Strikethrough
- Blockquotes
- Horizontal Rules
- Images
- HTML

# Assumptions

It is assumed that the Markdown is correctly formatted.
