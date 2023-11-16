# About

Convert markdown to pandoc markdown with custom styles

# Usage

```text
$ stylin -h
Convert markdown to pandoc markdown with custom styles

Usage: stylin [OPTIONS] [PATH]...

Arguments:
  [PATH]...  Input file(s); use `-` for stdin

Options:
  -c <PATH>      Configuration file [default: stylin.json]
  -r             Print readme
  -h, --help     Print help
  -V, --version  Print version
```

```text
$ stylin -V
stylin 0.1.1
```

# Example

Given a configuration file, [`stylin.json`](stylin.json):

~~~json5
{
  // Spans
  code: "Custom Code Style Name",
  emphasis: "Custom Emphasis Style Name",
  strong: "Custom Strong Style Name",
  strong_emphasis: "Custom Strong Emphasis Style Name",
  strong_code: "Custom Strong Code Style Name",

  // Blocks
  heading_1: "Custom Heading Level 1 Style Name",
  heading_2: "Custom Heading Level 2 Style Name",
  heading_3: "Custom Heading Level 3 Style Name",
  heading_4: "Custom Heading Level 4 Style Name",
  heading_5: "Custom Heading Level 5 Style Name",
  heading_6: "Custom Heading Level 6 Style Name",
  paragraph: "Custom Paragraph Style Name",
  ordered_list: "Custom Ordered List Style Name",
  unordered_list: "Custom Unordered List Style Name",
  table: "Custom Table Style Name",
  blockquote: "Custom Blockquote Style Name",
  fenced_code_block: "Custom Fenced Code Block Style Name",
  indented_code_block: "Custom Indented Code Block Style Name",

  // Other
  convert_smart_quotes: false,
}
~~~

And an input file, [`input.md`](input.md):

~~~md
# Heading level 1

This is a paragraph.

Here's a paragraph with a `code span`.

Another paragraph with two code spans: `alpha` and `bravo`.

Here's a code span with a backtick: ``code span with ` backtick``.

* Unordered list item 1

  Paragraph

* Unordered list item 2

1. Ordered list item 1
2. Ordered list item 2

## Heading level 2

A | B
---:|:---
1 | 2
2 | 4
3 | 8
4 | 16

### Heading level 3

Paragraph with
*emphasis text*,
**strong text**,
***strong emphasis text***, and
**`strong code text`**.

#### Heading level 4

    1: Indented code block
    2: 
    3: Blah

##### Heading level 5

> Blockquote paragraph
>
> > Nested blockquote paragraph

###### Heading level 6

```info
1: Fenced code block
2: 
3: Blah
```

~~~

Run the stylin command:

```bash
stylin input.md >output.md
```

To produce the output file, [`output.md`](output.md):

~~~md
:::{custom-style="Custom Heading Level 1 Style Name"}
Heading level 1
:::

:::{custom-style="Custom Paragraph Style Name"}
This is a paragraph.
:::

:::{custom-style="Custom Paragraph Style Name"}
Here's a paragraph with a [code span]{custom-style="Custom Code Style Name"}.
:::

:::{custom-style="Custom Paragraph Style Name"}
Another paragraph with two code spans: [alpha]{custom-style="Custom Code Style Name"} and [bravo]{custom-style="Custom Code Style Name"}.
:::

:::{custom-style="Custom Paragraph Style Name"}
Here's a code span with a backtick: [code span with ` backtick]{custom-style="Custom Code Style Name"}.
:::

:::{custom-style="Custom Unordered List Style Name"}
* Unordered list item 1

  Paragraph

* Unordered list item 2
:::

:::{custom-style="Custom Ordered List Style Name"}
1. Ordered list item 1

1. Ordered list item 2
:::

:::{custom-style="Custom Heading Level 2 Style Name"}
Heading level 2
:::

:::{custom-style="Custom Table Style Name"}
A | B
---:|:---
1 | 2
2 | 4
3 | 8
4 | 16
:::

:::{custom-style="Custom Heading Level 3 Style Name"}
Heading level 3
:::

:::{custom-style="Custom Paragraph Style Name"}
Paragraph with
[emphasis text]{custom-style="Custom Emphasis Style Name"},
[strong text]{custom-style="Custom Strong Style Name"},
[strong emphasis text]{custom-style="Custom Strong Emphasis Style Name"}, and
[[strong code text]{custom-style="Custom Code Style Name"}]{custom-style="Custom Strong Style Name"}.
:::

:::{custom-style="Custom Heading Level 4 Style Name"}
Heading level 4
:::

:::{custom-style="Custom Indented Code Block Style Name"}
1: Indented code block

2: 

3: Blah
:::

:::{custom-style="Custom Heading Level 5 Style Name"}
Heading level 5
:::

:::{custom-style="Custom Blockquote Style Name"}
Blockquote paragraph

> Nested blockquote paragraph
:::

:::{custom-style="Custom Heading Level 6 Style Name"}
Heading level 6
:::

:::{custom-style="Custom Fenced Code Block Style Name"}
1: Fenced code block

2: 

3: Blah
:::

~~~

# Changelog

* 0.1.0 (2023-11-16) Initial release
* 0.1.1 (2023-11-16) Remove null configuration file; fix bad json syntax
  highlighting on readme

# Notes

1. Conversion is good enough to reproduce the original intended content, but it
   does not worry about factors like list bullets / numbers.
   As a result, output from a *null configuration* (`{}`), will not be identical
   to the input, but what's the point of a null configuration? 

2. Block styles are applied to outermost blocks only.

3. Table style works but is broken in pandoc versions greater than 2.7.1 (see
   [jgm/pandoc#6496](https://github.com/jgm/pandoc/issues/6496)).

4. So-called "double styles" (`strong_emphasis`, `strong_code`) each require the
   two base styles also be defined.

