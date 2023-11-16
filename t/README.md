# About

Convert markdown to pandoc markdown with custom styles

# Usage

```text
$ stylin -h
!run:../target/release/stylin -h
```

```text
$ stylin -V
!run:../target/release/stylin -V
```

# Example

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
stylin input.md >output.md
!run:../target/release/stylin -c ../stylin.json ../input.md >../output.md 2>temp \
&& cat temp && rm temp
```

To produce the output file, [`output.md`](output.md):

~~~md
!inc:../output.md
~~~

!inc:../CHANGELOG.md

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

