# About

Convert markdown to pandoc markdown with custom styles

Stylin provides a [library](#library) and a [command line utility](#cli).

# Pipeline

Stylin can be used in a pipeline with other tools, like [mkrs], [kapow],
[pandoc], etc.

[mkrs]: https://crates.io/crates/mkrs
[kapow]: https://crates.io/crates/kapow
[pandoc]: https://pandoc.org

![](pipeline.png)

# CLI

## Usage

```text
$ stylin -h
!run:../target/release/stylin -h
```

```text
$ stylin -V
!run:../target/release/stylin -V
```

## Example

Given a configuration file, [`stylin.json`](stylin.json):

~~~json5
!inc:../stylin.json
~~~

And an input file, [`input.md`](input.md):

~~~md
!inc:../input.md
~~~

Run the stylin command:

```bash
stylin input.md >stylin.md
```

To produce the output file, [`stylin.md`](stylin.md):

~~~text
!inc:../stylin.md
~~~

# Library

See the [documentation](https://docs.rs/stylin) for usage and an example.

!inc:../CHANGELOG.md

# Notes

1. Conversion is good enough to reproduce the original intended content, but it
   does not worry about factors like list bullets / numbers.
   As a result, output from a *null configuration* (`{}`), will not be identical
   to the input, but what's the point of a null configuration? 

2. Block styles are generally applied to outermost blocks only, except for the
   figure style (if enabled), which will be applied to figure list item
   paragraphs.

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

