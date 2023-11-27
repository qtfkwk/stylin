use super::*;
use lazy_static::lazy_static;

const CONFIG: &str = include_str!("../stylin.json");

lazy_static! {
    static ref STYLIN: Stylin = Stylin::from(CONFIG).unwrap();
}

#[test]
fn heading_1() {
    assert_eq!(
        STYLIN
            .convert(
                "\
# Heading level 1

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 1 Style Name\"}
Heading level 1
:::

\
            ",
        ],
    );
}

#[test]
fn heading_2() {
    assert_eq!(
        STYLIN
            .convert(
                "\
## Heading level 2

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 2 Style Name\"}
Heading level 2
:::

\
            ",
        ],
    );
}

#[test]
fn heading_3() {
    assert_eq!(
        STYLIN
            .convert(
                "\
### Heading level 3

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 3 Style Name\"}
Heading level 3
:::

\
            ",
        ],
    );
}

#[test]
fn heading_4() {
    assert_eq!(
        STYLIN
            .convert(
                "\
#### Heading level 4

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 4 Style Name\"}
Heading level 4
:::

\
            ",
        ],
    );
}

#[test]
fn heading_5() {
    assert_eq!(
        STYLIN
            .convert(
                "\
##### Heading level 5

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 5 Style Name\"}
Heading level 5
:::

\
            ",
        ],
    );
}

#[test]
fn heading_6() {
    assert_eq!(
        STYLIN
            .convert(
                "\
###### Heading level 6

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Heading Level 6 Style Name\"}
Heading level 6
:::

\
            ",
        ],
    );
}

#[test]
fn paragraph() {
    assert_eq!(
        STYLIN
            .convert(
                "\
This is a paragraph.

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
This is a paragraph.
:::

\
            ",
        ],
    );
}

#[test]
fn paragraph_with_code_span() {
    assert_eq!(
        STYLIN
            .convert(
                "\
Here's a paragraph with a `code span`.

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
Here's a paragraph with a [code span]{custom-style=\"Custom Code Style Name\"}.
:::

\
            ",
        ],
    );
}

#[test]
fn paragraph_with_code_spans() {
    assert_eq!(
        STYLIN
            .convert(
                "\
Another paragraph with two code spans: `alpha` and `bravo`.

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
Another paragraph with two code spans: [alpha]{custom-style=\"Custom Code Style Name\"} and \
[bravo]{custom-style=\"Custom Code Style Name\"}.
:::

\
            ",
        ],
    );
}

#[test]
fn code_span_with_backtick() {
    assert_eq!(
        STYLIN
            .convert(
                "\
Here's a code span with a backtick: ``code span with ` backtick``.

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
Here's a code span with a backtick: \
[code span with ` backtick]{custom-style=\"Custom Code Style Name\"}.
:::

\
            ",
        ],
    );
}

#[test]
fn unordered_list() {
    assert_eq!(
        STYLIN
            .convert(
                "\
* Unordered list item 1

  Paragraph

* Unordered list item 2

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Unordered List Style Name\"}
* Unordered list item 1

  Paragraph

* Unordered list item 2
:::

\
            ",
        ],
    );
}

#[test]
fn ordered_list() {
    assert_eq!(
        STYLIN
            .convert(
                "\
1. Ordered list item 1
2. Ordered list item 2

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Ordered List Style Name\"}
1. Ordered list item 1

1. Ordered list item 2
:::

\
            ",
        ],
    );
}

#[test]
fn table() {
    assert_eq!(
        STYLIN
            .convert(
                "\
A | B
---:|:---
1 | 2
2 | 4
3 | 8
4 | 16

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Table Style Name\"}
A | B
---:|:---
1 | 2
2 | 4
3 | 8
4 | 16
:::

\
            ",
        ],
    );
}

#[test]
fn paragraph_with_emphasis_strong() {
    assert_eq!(
        STYLIN
            .convert(
                "\
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

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
Paragraph with
[emphasis text 1]{custom-style=\"Custom Emphasis Style Name\"},
[emphasis text 2]{custom-style=\"Custom Emphasis Style Name\"},
[strong text 1]{custom-style=\"Custom Strong Style Name\"},
[strong text 2]{custom-style=\"Custom Strong Style Name\"},
[emphasis strong text 1]{custom-style=\"Custom Emphasis Strong Style Name\"},
[emphasis strong text 2]{custom-style=\"Custom Emphasis Strong Style Name\"},
[emphasis strong text 3]{custom-style=\"Custom Emphasis Strong Style Name\"},
[emphasis strong text 4]{custom-style=\"Custom Emphasis Strong Style Name\"},
[strong emphasis text 1]{custom-style=\"Custom Strong Emphasis Style Name\"},
[strong emphasis text 2]{custom-style=\"Custom Strong Emphasis Style Name\"}, and
[strong code text]{custom-style=\"Custom Strong Code Style Name\"}.
:::

\
            ",
        ],
    );
}

#[test]
fn indented_code_block() {
    assert_eq!(
        STYLIN
            .convert(
                "\
    1: Indented code block
    2:
    3: Blah

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
1: Indented code block
2:
3: Blah
:::

\
            ",
        ],
    );
}

#[test]
fn blockquote() {
    assert_eq!(
        STYLIN
            .convert(
                "\
> Blockquote paragraph
>
> > Nested blockquote paragraph

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Blockquote Style Name\"}
Blockquote paragraph

> Nested blockquote paragraph
:::

\
            ",
        ],
    );
}

#[test]
fn fenced_code_block() {
    assert_eq!(
        STYLIN
            .convert(
                "\
```info
1: Fenced code block
2:
3: Blah
```

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Fenced Code Block Style Name\"}
1: Fenced code block

2:

3: Blah
:::

\
            ",
        ],
    );
}

#[test]
fn paragraph_with_block_style() {
    let content = "\
:::{custom-style=\"Some Other Custom Style\"}
This paragraph already has a block style.
:::

\
    ";
    assert_eq!(STYLIN.convert(content).unwrap(), vec![content]);
}

#[test]
fn unordered_list_with_block_style() {
    let content = "\
:::{custom-style=\"Some Other Custom Style\"}
* Alpha

* Bravo

* Charlie
:::

\
    ";
    assert_eq!(STYLIN.convert(content).unwrap(), vec![content]);
}

#[test]
fn links() {
    for content in ["[text](url)", "[text](url \"title\")"] {
        assert_eq!(
            STYLIN.convert(content).unwrap(),
            vec![format!(
                ":::{{custom-style=\"Custom Paragraph Style Name\"}}\n{content}\n:::\n\n",
            )],
            "{content}"
        );
    }
}

#[test]
fn images() {
    for content in [
        "![](path)",
        "![alt](path)",
        "![](path \"title\")",
        "![alt](path \"title\")",
        "[![](path)](url)",
        "[![alt](path)](url)",
        "[![](path \"title\")](url)",
        "[![alt](path \"title\")](url)",
        "[![](path)](url \"title\")",
        "[![alt](path)](url \"title\")",
        "[![](path \"title\")](url \"title\")",
        "[![alt](path \"title\")](url \"title\")",
    ] {
        assert_eq!(
            STYLIN.convert(content).unwrap(),
            vec![format!(
                ":::{{custom-style=\"Custom Figure Style Name\"}}\n{content}\n:::\n\n",
            )],
            "{content}"
        );
    }
}

#[test]
fn unordered_list_with_image() {
    assert_eq!(
        STYLIN
            .convert(
                "\
* Unordered list item

  ![](path)

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Unordered List Style Name\"}
* Unordered list item

  ![](path)
:::

\
            ",
        ],
    );
}

#[test]
fn ordered_list_with_image() {
    assert_eq!(
        STYLIN
            .convert(
                "\
1. Ordered list item

   ![](path)

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Ordered List Style Name\"}
1. Ordered list item

   ![](path)
:::

\
            ",
        ],
    );
}

#[test]
fn table_with_image() {
    assert_eq!(
        STYLIN
            .convert(
                "\
A | B
---|---
1 | ![alt](path)

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Table Style Name\"}
A | B
---|---
1 | ![alt](path)
:::

\
            ",
        ],
    );
}

#[test]
fn blockquote_with_image() {
    assert_eq!(
        STYLIN
            .convert(
                "\
> ![alt](path)

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Blockquote Style Name\"}
![alt](path)
:::

\
            ",
        ],
    );
}

#[test]
fn blockquote_with_br() {
    assert_eq!(
        STYLIN
            .convert(
                "\
A | B
---|---
1 | 2<br>3

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Table Style Name\"}
A | B
---|---
1 | 2<br>3
:::

\
            ",
        ],
    );
}

#[test]
fn code_span_with_backslash() {
    assert_eq!(
        STYLIN
            .convert(
                "\
This code span has a backslash: `Code span with a \\ backslash`.

\
                ",
            )
            .unwrap(),
        vec![
            "\
:::{custom-style=\"Custom Paragraph Style Name\"}
This code span has a backslash: \
[Code span with a \\\\ backslash]{custom-style=\"Custom Code Style Name\"}.
:::

\
            ",
        ],
    );
}
