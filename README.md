# About

Convert markdown to pandoc markdown with custom styles

Stylin provides a [library](#library) and a [command line utility](#cli).

# CLI

## Usage

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
stylin 0.5.0
```

## Example

Given a configuration file, [`stylin.json`](stylin.json):

~~~json5
{
  // Spans
  code: "Custom Code Style Name",
  emphasis: "Custom Emphasis Style Name",
  strong: "Custom Strong Style Name",
  emphasis_strong: "Custom Emphasis Strong Style Name",
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
  figure: "Custom Figure Style Name",
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
*emphasis text 1*,
_emphasis text 2_,
**strong text 1**,
__strong text 2__,
***emphasis strong text 1***,
___emphasis strong text 2___,
_**emphasis strong text 3**_,
*__emphasis strong text 4__*,
**_strong emphasis text 1_**,
__*strong emphasis text 2*__, and
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

:::{custom-style="Some Other Custom Style"}
This paragraph already has a block style.
:::

:::{custom-style="Some Other Custom Style"}
* Alpha

* Bravo

* Charlie
:::

![](path)

![alt](path)

![](path "title")

![alt](path "title")

[text](url)

[text](url "title")

[![](path)](url)

[![alt](path)](url)

[![](path "title")](url)

[![alt](path "title")](url)

[![](path)](url "title")

[![alt](path)](url "title")

[![](path "title")](url "title")

[![alt](path "title")](url "title")

Alpha [![alt](path "title")](url "title") bravo.

* Unordered list item

  ![](path)

1. Ordered list item

   ![](path)

A | B
---|---
1 | ![alt](path)

> ![alt](path)

A | B
---|---
1 | 2<br>3

This code span has a backslash: `Code span with a \ backslash`.

~~~

Run the stylin command:

```bash
stylin input.md >output.md
```

To produce the output file, [`output.md`](output.md):

~~~text
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
[emphasis text 1]{custom-style="Custom Emphasis Style Name"},
[emphasis text 2]{custom-style="Custom Emphasis Style Name"},
[strong text 1]{custom-style="Custom Strong Style Name"},
[strong text 2]{custom-style="Custom Strong Style Name"},
[emphasis strong text 1]{custom-style="Custom Emphasis Strong Style Name"},
[emphasis strong text 2]{custom-style="Custom Emphasis Strong Style Name"},
[emphasis strong text 3]{custom-style="Custom Emphasis Strong Style Name"},
[emphasis strong text 4]{custom-style="Custom Emphasis Strong Style Name"},
[strong emphasis text 1]{custom-style="Custom Strong Emphasis Style Name"},
[strong emphasis text 2]{custom-style="Custom Strong Emphasis Style Name"}, and
[strong code text]{custom-style="Custom Strong Code Style Name"}.
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

:::{custom-style="Some Other Custom Style"}
This paragraph already has a block style.
:::

:::{custom-style="Some Other Custom Style"}
* Alpha

* Bravo

* Charlie
:::

:::{custom-style="Custom Figure Style Name"}
![](path)
:::

:::{custom-style="Custom Figure Style Name"}
![alt](path)
:::

:::{custom-style="Custom Figure Style Name"}
![](path "title")
:::

:::{custom-style="Custom Figure Style Name"}
![alt](path "title")
:::

:::{custom-style="Custom Paragraph Style Name"}
[text](url)
:::

:::{custom-style="Custom Paragraph Style Name"}
[text](url "title")
:::

:::{custom-style="Custom Figure Style Name"}
[![](path)](url)
:::

:::{custom-style="Custom Figure Style Name"}
[![alt](path)](url)
:::

:::{custom-style="Custom Figure Style Name"}
[![](path "title")](url)
:::

:::{custom-style="Custom Figure Style Name"}
[![alt](path "title")](url)
:::

:::{custom-style="Custom Figure Style Name"}
[![](path)](url "title")
:::

:::{custom-style="Custom Figure Style Name"}
[![alt](path)](url "title")
:::

:::{custom-style="Custom Figure Style Name"}
[![](path "title")](url "title")
:::

:::{custom-style="Custom Figure Style Name"}
[![alt](path "title")](url "title")
:::

:::{custom-style="Custom Paragraph Style Name"}
Alpha [![alt](path "title")](url "title") bravo.
:::

:::{custom-style="Custom Unordered List Style Name"}
* Unordered list item

  ![](path)
:::

:::{custom-style="Custom Ordered List Style Name"}
1. Ordered list item

   ![](path)
:::

:::{custom-style="Custom Table Style Name"}
A | B
---|---
1 | ![alt](path)
:::

:::{custom-style="Custom Blockquote Style Name"}
![alt](path)
:::

:::{custom-style="Custom Table Style Name"}
A | B
---|---
1 | 2<br>3
:::

:::{custom-style="Custom Paragraph Style Name"}
This code span has a backslash: [Code span with a \\ backslash]{custom-style="Custom Code Style Name"}.
:::

~~~

# Library

See the [documentation](https://docs.rs/stylin) for usage and an example.

# Changelog

* 0.1.0 (2023-11-16): Initial release
* 0.1.1 (2023-11-16): Remove null configuration file; fix bad json syntax
  highlighting on readme
* 0.2.0 (2023-11-17): Add doctest; add `emphasis_strong` double style; add
  more strong/emphasis examples to `input.md`; fix issue with strong code double
  style; improve the double style algorithm; fix changelog; improve readme
* 0.3.0 (2023-11-17): Avoid adding block style if it already has a style; add
  images; remove convert smart quotes feature
* 0.4.0 (2023-11-18): Fix image alt text; add links; fix comments; add unit
  tests
* 0.4.1 (2023-11-20): Fix image/link depth issue
* 0.4.2 (2023-11-20): Fix image/link and/or other content in table or blockquote
  issue; fix image/link depth tests
* 0.4.3 (2023-11-20): Add note #5 to avoid using a single style for blocks and
  spans; properly handle backslashes in styled code spans
* 0.5.0 (2023-11-27): Add `figure` style; update dependencies

# Notes

1. Conversion is good enough to reproduce the original intended content, but it
   does not worry about factors like list bullets / numbers.
   As a result, output from a *null configuration* (`{}`), will not be identical
   to the input, but what's the point of a null configuration? 

2. Block styles are applied to outermost blocks only.

3. Table style works but is broken in pandoc versions greater than 2.7.1 (see
   [jgm/pandoc#6496](https://github.com/jgm/pandoc/issues/6496)).

4. So-called "double styles" (`emphasis_strong`, `strong_emphasis`, and
   `strong_code`) each require the base styles also be defined.

5. Never define a single style name to be used by both blocks and spans!
   If you do and a document uses it, there will be conflicts later in the
   pipeline.
   For instance, in Microsoft Word, the style will be *upgraded* to a block
   style, so a span that uses it inside a block will override the correct block
   style.
   It is best to define and use separate styles for blocks and spans.

