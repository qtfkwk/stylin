/*!

```
use stylin::Stylin;

let s = Stylin::from(r#"{
    heading_1: "Heading Level 1 Style",
    paragraph: "Paragraph Style",
}"#).expect("config");

let input = String::from(
    "# This is a heading\n\nThis is a paragraph.\n\n",
);

let output = s.convert(&input).expect("convert");

assert_eq!(
  output.join(""),
  r#":::{custom-style="Heading Level 1 Style"}
This is a heading
:::

:::{custom-style="Paragraph Style"}
This is a paragraph.
:::

"#,
);
```

*/

//--------------------------------------------------------------------------------------------------

use anyhow::Result;
use pulldown_cmark as pd;
use serde::Deserialize;
use std::fmt::Write;
use std::path::Path;

//--------------------------------------------------------------------------------------------------

/**

Primary interface

See the module level documentation for an example.

*/
#[derive(Debug, Default, Deserialize)]
pub struct Stylin {
    // Spans
    code: Option<String>,
    emphasis: Option<String>,
    strong: Option<String>,
    emphasis_strong: Option<String>,
    strong_emphasis: Option<String>,
    strong_code: Option<String>,

    // Blocks
    heading_1: Option<String>,
    heading_2: Option<String>,
    heading_3: Option<String>,
    heading_4: Option<String>,
    heading_5: Option<String>,
    heading_6: Option<String>,
    paragraph: Option<String>,
    ordered_list: Option<String>,
    unordered_list: Option<String>,
    blockquote: Option<String>,
    fenced_code_block: Option<String>,
    indented_code_block: Option<String>,
    table: Option<String>,

    // Other
    #[serde(default = "default_config_convert_smart_quotes")]
    convert_smart_quotes: bool,
}

fn default_config_convert_smart_quotes() -> bool {
    false
}

impl Stylin {
    /**

    Instantiate from a string slice

    */
    pub fn from(s: &str) -> Result<Stylin> {
        let r: Stylin = json5::from_str(s)?;
        Ok(r)
    }

    /**

    Instantiate from a file path

    */
    pub fn from_path(path: &Path) -> Result<Stylin> {
        Stylin::from(&std::fs::read_to_string(path)?)
    }

    /**

    Convert markdown to pandoc markdown

    */
    pub fn convert(&self, input: &str) -> Result<Vec<String>> {
        let mut blocks = vec![];

        // State
        let mut depth = 0;
        let mut lists = vec![];
        let mut first_li = false;
        let mut first_li_p = false;
        let mut indents = vec![];
        let mut disabled = false;
        let mut block = String::new();

        for (event, range) in pd::Parser::new_ext(input, pd::Options::all()).into_offset_iter() {
            let source = &input[range.clone()];

            match event {
                pd::Event::Start(tag) => match tag {
                    pd::Tag::Emphasis => {
                        if self.emphasis.is_some() {
                            write!(block, "[")?;
                        } else {
                            write!(block, "*")?;
                        }
                    }
                    pd::Tag::Strong => {
                        if self.strong.is_some() {
                            write!(block, "[")?;
                        } else {
                            write!(block, "**")?;
                        }
                    }
                    pd::Tag::Heading(level, ..) => {
                        let mut done = false;
                        if depth == 0 {
                            if let Some(style) = match level {
                                pd::HeadingLevel::H1 => &self.heading_1,
                                pd::HeadingLevel::H2 => &self.heading_2,
                                pd::HeadingLevel::H3 => &self.heading_3,
                                pd::HeadingLevel::H4 => &self.heading_4,
                                pd::HeadingLevel::H5 => &self.heading_5,
                                pd::HeadingLevel::H6 => &self.heading_6,
                            } {
                                writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                done = true;
                            }
                        }
                        if !done {
                            write!(
                                block,
                                "{} ",
                                "#".repeat(match level {
                                    pd::HeadingLevel::H1 => 1,
                                    pd::HeadingLevel::H2 => 2,
                                    pd::HeadingLevel::H3 => 3,
                                    pd::HeadingLevel::H4 => 4,
                                    pd::HeadingLevel::H5 => 5,
                                    pd::HeadingLevel::H6 => 6,
                                })
                            )?;
                        }
                        depth += 1;
                    }
                    pd::Tag::Paragraph => {
                        if !disabled {
                            if let Some(None) = lists.last() {
                                if !first_li_p {
                                    write!(block, "\n\n{}", indents.join(""))?;
                                }
                                first_li_p = false;
                            } else if let Some(Some(_n)) = lists.last() {
                                if !first_li_p {
                                    write!(block, "\n\n{}", indents.join(""))?;
                                }
                                first_li_p = false;
                            } else if depth == 0 {
                                if let Some(style) = &self.paragraph {
                                    writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                }
                            }
                        }
                        depth += 1;
                    }
                    pd::Tag::List(t) => {
                        if lists.is_empty() {
                            if t.is_some() {
                                if let Some(style) = &self.ordered_list {
                                    writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                }
                                indents.push("   ");
                            } else {
                                if let Some(style) = &self.unordered_list {
                                    writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                }
                                indents.push("  ");
                            }
                        }
                        lists.push(t);
                        first_li = true;
                        first_li_p = true;
                        depth += 1;
                    }
                    pd::Tag::Item => {
                        if !first_li {
                            writeln!(block,)?;
                        }
                        first_li = false;
                        match lists.last().unwrap() {
                            Some(n) => {
                                write!(block, "{n}. ")?;
                            }
                            None => {
                                write!(block, "* ")?;
                            }
                        }
                        first_li_p = true;
                        depth += 1;
                    }
                    pd::Tag::BlockQuote | pd::Tag::Table(_) | pd::Tag::CodeBlock(_) => {
                        disabled = true;
                        depth += 1;
                    }
                    _ => {} // end tags
                }, // end start
                pd::Event::Code(s) => {
                    if !disabled {
                        let mut done = false;
                        if let Some(style) = &self.code {
                            write!(block, "[{s}]{{custom-style=\"{style}\"}}")?;
                            done = true;
                        }
                        if !done {
                            write!(block, "{source}")?;
                        }
                    }
                }
                pd::Event::Text(s) => {
                    if !disabled {
                        if self.convert_smart_quotes {
                            write!(block, "{s}")?;
                        } else {
                            write!(block, "{source}")?;
                        }
                    }
                }
                pd::Event::Rule => {
                    if !disabled {
                        writeln!(block, "---\n")?;
                    }
                }
                pd::Event::End(tag) => match tag {
                    pd::Tag::Emphasis => {
                        if let Some(style) = &self.emphasis {
                            write!(block, "]{{custom-style=\"{style}\"}}")?;
                        } else {
                            write!(block, "*")?;
                        }
                    }
                    pd::Tag::Strong => {
                        if let Some(style) = &self.strong {
                            write!(block, "]{{custom-style=\"{style}\"}}")?;
                        } else {
                            write!(block, "**")?;
                        }
                    }

                    // Blocks
                    pd::Tag::Heading(level, ..) => {
                        depth -= 1;
                        if depth == 0 {
                            if match level {
                                pd::HeadingLevel::H1 => &self.heading_1,
                                pd::HeadingLevel::H2 => &self.heading_2,
                                pd::HeadingLevel::H3 => &self.heading_3,
                                pd::HeadingLevel::H4 => &self.heading_4,
                                pd::HeadingLevel::H5 => &self.heading_5,
                                pd::HeadingLevel::H6 => &self.heading_6,
                            }
                            .is_some()
                            {
                                writeln!(block, "\n:::\n")?;
                            } else {
                                writeln!(block, "\n")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                    }
                    pd::Tag::Paragraph => {
                        depth -= 1;
                        if !disabled && lists.is_empty() && depth == 0 {
                            if self.paragraph.is_some() {
                                writeln!(block, "\n:::\n")?;
                            } else {
                                writeln!(block, "\n")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                    }
                    pd::Tag::List(t) => {
                        depth -= 1;
                        if depth == 0 {
                            if t.is_some() {
                                if self.ordered_list.is_some() {
                                    writeln!(block, ":::\n")?;
                                } else {
                                    writeln!(block)?;
                                }
                            } else if self.unordered_list.is_some() {
                                writeln!(block, ":::\n")?;
                            } else {
                                writeln!(block)?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        let _ = lists.pop().unwrap();
                        let _ = indents.pop().unwrap();
                    }
                    pd::Tag::Item => {
                        depth -= 1;
                        writeln!(block)?;
                    }
                    pd::Tag::BlockQuote => {
                        depth -= 1;
                        if depth == 0 {
                            if let Some(style) = &self.blockquote {
                                let content = source
                                    .lines()
                                    .map(|x| {
                                        x.strip_prefix("> ").unwrap_or(x.strip_prefix('>').unwrap())
                                    })
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                writeln!(
                                    block,
                                    ":::{{custom-style=\"{style}\"}}\n{content}\n:::\n"
                                )?;
                            } else {
                                writeln!(block, "{source}")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        disabled = false;
                    }
                    pd::Tag::Table(_) => {
                        depth -= 1;
                        if depth == 0 {
                            if let Some(style) = &self.table {
                                writeln!(block, ":::{{custom-style=\"{style}\"}}\n{source}:::\n")?;
                            } else {
                                writeln!(block, "{source}")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        disabled = false;
                    }
                    pd::Tag::CodeBlock(pd::CodeBlockKind::Fenced(_)) => {
                        depth -= 1;
                        if depth == 0 {
                            if let Some(style) = &self.fenced_code_block {
                                let mut content = source.lines().collect::<Vec<_>>();
                                content.remove(0);
                                content.pop().unwrap();
                                let content = content.join("\n\n");
                                writeln!(
                                    block,
                                    ":::{{custom-style=\"{style}\"}}\n{content}\n:::\n"
                                )?;
                            } else {
                                writeln!(block, "{source}")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        disabled = false;
                    }
                    pd::Tag::CodeBlock(pd::CodeBlockKind::Indented) => {
                        depth -= 1;
                        if depth == 0 {
                            if let Some(style) = &self.indented_code_block {
                                let content = source
                                    .lines()
                                    .map(|x| x.strip_prefix("    ").unwrap_or(x))
                                    .collect::<Vec<_>>()
                                    .join("\n\n");
                                writeln!(
                                    block,
                                    ":::{{custom-style=\"{style}\"}}\n{content}\n:::\n"
                                )?;
                            } else {
                                writeln!(block, "    {source}")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        disabled = false;
                    }
                    _ => {}
                },
                _ => {
                    write!(block, "{source}")?;
                } // end tags
            } // end end
        } // end events

        Ok(blocks)
    }

    /**

    Perform postprocessing on the block content then add it to `blocks`

    * Resolve "double styles" (`strong_emphasis`, `strong_code`)

    */
    fn process_block(&self, mut block: String, blocks: &mut Vec<String>) -> String {
        for (outer, inner, double) in [
            (&self.emphasis, &self.strong, &self.emphasis_strong),
            (&self.strong, &self.emphasis, &self.strong_emphasis),
            (&self.strong, &self.code, &self.strong_code),
        ] {
            resolve_double_style(outer, inner, double, &mut block);
        }
        blocks.push(block);
        String::new()
    }
}

//--------------------------------------------------------------------------------------------------

/**

Replace "double style" spans with a single style

*/
fn resolve_double_style(
    outer: &Option<String>,
    inner: &Option<String>,
    double: &Option<String>,
    block: &mut String,
) {
    if let Some(style_outer) = outer {
        if let Some(style_inner) = inner {
            if let Some(style_double) = double {
                // All three styles must be defined...

                let replace = format!(
                    "]{{custom-style=\"{style_inner}\"}}]{{custom-style=\"{style_outer}\"}}"
                );
                let with = format!("]{{custom-style=\"{style_double}\"}}");
                let loss = replace.len() - with.len() + 1;

                let mut i = 0;
                while let Some(mut c) = block[i..].find(&replace) {
                    c += i;
                    let d = c + replace.len();
                    if let Some(a) = block[..c].rfind("[[") {
                        let b = a + 2;
                        block.replace_range(c..d, &with);
                        block.replace_range(a..b, "[");
                    }
                    i = d - loss;
                }
            }
        }
    }
}
