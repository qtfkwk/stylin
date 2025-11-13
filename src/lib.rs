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
// Crates

use {
    anyhow::Result,
    pulldown_cmark as pd,
    regex::Regex,
    serde::Deserialize,
    std::{fmt::Write, path::Path, sync::LazyLock},
};

//--------------------------------------------------------------------------------------------------
// Modules

#[cfg(test)]
mod tests;

//--------------------------------------------------------------------------------------------------
// Statics

static FIGURE_CAPTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Figure [0-9\.]+:").unwrap());
static LISTING_CAPTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^Listing [0-9\.]+:").unwrap());
static TABLE_CAPTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^Table [0-9\.]+:").unwrap());

//--------------------------------------------------------------------------------------------------
// Structs

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
    table: Option<String>,
    blockquote: Option<String>,
    fenced_code_block: Option<String>,
    indented_code_block: Option<String>,
    figure: Option<String>,
    figure_caption: Option<String>,
    table_caption: Option<String>,
    listing_caption: Option<String>,

    // Other
    #[serde(default = "default_debug")]
    debug: bool,
}

fn default_debug() -> bool {
    false
}

impl Stylin {
    /**
    Instantiate from a string slice

    # Errors

    Returns an error if not able to parse the given `&str` as JSON5
    */
    pub fn from(s: &str) -> Result<Stylin> {
        let r: Stylin = json5::from_str(s)?;
        Ok(r)
    }

    /**
    Instantiate from a file path

    # Errors

    Returns an error if not able to read the file at the given path to a string
    */
    pub fn from_path(path: &Path) -> Result<Stylin> {
        Stylin::from(&std::fs::read_to_string(path)?)
    }

    /**
    Convert markdown to pandoc markdown
    */
    #[allow(
        clippy::missing_errors_doc,
        clippy::missing_panics_doc,
        clippy::too_many_lines
    )]
    pub fn convert(&self, input: &str) -> Result<Vec<String>> {
        let mut blocks: Vec<String> = vec![];

        // State
        let mut depth = 0;
        let mut lists = vec![];
        let mut first_li_p = false;
        let mut li_p = false;
        let mut end_figure_div = false;
        let mut indents = vec![];
        let mut disabled = false;
        let mut block = String::new();
        let mut paragraph = None;
        let mut img_link_depth = 0;
        let mut info = None;

        let mut options = pd::Options::all();
        options.remove(pd::Options::ENABLE_DEFINITION_LIST);

        for (event, range) in pd::Parser::new_ext(input, options).into_offset_iter() {
            let source = &input[range.clone()];

            if self.debug {
                eprintln!("---");
                eprintln!("range = {range:?}");
                eprintln!("source = {source:?}");
                eprintln!("event = {event:?}");
                eprintln!("lists = {lists:?}");
            }

            match event {
                // Start tags
                pd::Event::Start(tag) => match tag {
                    // Spans
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
                    pd::Tag::Image { .. } | pd::Tag::Link { .. } => {
                        img_link_depth += 1;
                    }

                    // Blocks
                    pd::Tag::Heading { level, .. } => {
                        let mut done = false;
                        if depth == 0
                            && let Some(style) = match level {
                                pd::HeadingLevel::H1 => &self.heading_1,
                                pd::HeadingLevel::H2 => &self.heading_2,
                                pd::HeadingLevel::H3 => &self.heading_3,
                                pd::HeadingLevel::H4 => &self.heading_4,
                                pd::HeadingLevel::H5 => &self.heading_5,
                                pd::HeadingLevel::H6 => &self.heading_6,
                            }
                        {
                            writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                            done = true;
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
                                    if !block.ends_with("\n\n") {
                                        writeln!(block, "\n")?;
                                    }
                                    write!(block, "{}", indents.join(""))?;
                                }
                                li_p = true;
                                first_li_p = false;
                            } else if let Some(Some(_n)) = lists.last() {
                                if !first_li_p {
                                    if !block.ends_with("\n\n") {
                                        writeln!(block, "\n")?;
                                    }
                                    write!(block, "{}", indents.join(""))?;
                                }
                                li_p = true;
                                first_li_p = false;
                            } else if depth == 0
                                && let Some(style) = &self.paragraph
                            {
                                paragraph = Some((style, false));
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
                            } else if let Some(style) = &self.unordered_list {
                                writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                            }
                        }
                        if paragraph.is_some() {
                            let prev_block = blocks.pop().unwrap();
                            block = prev_block.strip_suffix(":::\n\n").unwrap().to_string();
                        }
                        lists.push(t);
                        indents.push(" ".repeat(if t.is_some() { 3 } else { 2 }));
                        first_li_p = true;
                        depth += 1;
                    }
                    pd::Tag::Item => {
                        while !block.ends_with("\n\n") {
                            writeln!(block)?;
                        }
                        if lists.len() > 1 {
                            write!(block, "{}", indents[..(indents.len() - 1)].join(""))?;
                        }
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
                    pd::Tag::CodeBlock(kind) => {
                        if let pd::CodeBlockKind::Fenced(s) = kind {
                            info = Some(s.to_string());
                        }
                        disabled = true;
                        depth += 1;
                    }
                    pd::Tag::BlockQuote(_) | pd::Tag::Table(_) => {
                        disabled = true;
                        depth += 1;
                    }
                    _ => {} // end tags
                }, // end start

                // Other events
                pd::Event::Code(s) => {
                    if !disabled && img_link_depth == 0 {
                        let mut done = false;
                        if let Some((style, printed)) = paragraph {
                            if printed {
                                write!(block, "{source}")?;
                            } else {
                                writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                paragraph = Some((style, true));
                            }
                        } else if let Some(style) = &self.code {
                            write!(
                                block,
                                "[{}]{{custom-style=\"{style}\"}}",
                                s.replace('\\', "\\\\")
                            )?;
                            done = true;
                        }
                        if !done {
                            write!(block, "{source}")?;
                        }
                    }
                }
                pd::Event::Text(_s) => {
                    if !disabled && img_link_depth == 0 {
                        if let Some((style, printed)) = paragraph {
                            if printed {
                                write!(block, "{source}")?;
                                if block.ends_with(":::") {
                                    paragraph = None;
                                }
                            } else {
                                if source.starts_with(":::") {
                                    paragraph = Some((style, true));
                                } else {
                                    let style = if source.starts_with("Figure:")
                                        || FIGURE_CAPTION.is_match(source)
                                    {
                                        if let Some(caption_style) = &self.figure_caption {
                                            caption_style
                                        } else {
                                            style
                                        }
                                    } else if source.starts_with("Table:")
                                        || TABLE_CAPTION.is_match(source)
                                    {
                                        if let Some(caption_style) = &self.table_caption {
                                            caption_style
                                        } else {
                                            style
                                        }
                                    } else if source.starts_with("Listing:")
                                        || LISTING_CAPTION.is_match(source)
                                    {
                                        if let Some(caption_style) = &self.listing_caption {
                                            caption_style
                                        } else {
                                            style
                                        }
                                    } else {
                                        style
                                    };
                                    writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                                    paragraph = None;
                                }
                                write!(block, "{source}")?;
                            }
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
                pd::Event::TaskListMarker(_) => {
                    write!(block, "{source} ")?;
                }

                // End tags
                pd::Event::End(tag) => match tag {
                    // Spans
                    pd::TagEnd::Emphasis => {
                        if let Some(style) = &self.emphasis {
                            write!(block, "]{{custom-style=\"{style}\"}}")?;
                        } else {
                            write!(block, "*")?;
                        }
                    }
                    pd::TagEnd::Strong => {
                        if let Some(style) = &self.strong {
                            write!(block, "]{{custom-style=\"{style}\"}}")?;
                        } else {
                            write!(block, "**")?;
                        }
                    }
                    pd::TagEnd::Image => {
                        if li_p && self.figure.is_some() {
                            let indent = indents.join("");
                            if !block.ends_with(&format!("\n{indent}")) {
                                write!(block, "{indent}")?;
                            }
                            let style = self.figure.as_ref().unwrap();
                            write!(block, ":::{{custom-style=\"{style}\"}}\n{indent}")?;
                            end_figure_div = true;
                        } else if let Some((style, false)) = paragraph {
                            let style = self.figure.as_ref().unwrap_or(style);
                            writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                            paragraph = Some((style, true));
                        }
                        img_link_depth -= 1;
                        if !disabled && img_link_depth == 0 {
                            write!(block, "{source}")?;
                        }
                    }
                    pd::TagEnd::Link => {
                        if let Some((style, false)) = paragraph {
                            writeln!(block, ":::{{custom-style=\"{style}\"}}")?;
                            paragraph = Some((style, true));
                        }
                        img_link_depth -= 1;
                        if !disabled && img_link_depth == 0 {
                            write!(block, "{source}")?;
                        }
                    }

                    // Blocks
                    pd::TagEnd::Heading(level, ..) => {
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
                    pd::TagEnd::Paragraph => {
                        depth -= 1;
                        if !disabled && lists.is_empty() && depth == 0 {
                            if self.paragraph.is_some() && !block.ends_with(":::") {
                                writeln!(block, "\n:::\n")?;
                            } else {
                                writeln!(block, "\n")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        }
                        if end_figure_div {
                            writeln!(block, "\n{}:::\n", indents.join(""))?;
                            end_figure_div = false;
                        }
                        li_p = false;
                    }
                    pd::TagEnd::List(t) => {
                        depth -= 1;
                        if depth == 0 {
                            if !block.ends_with("\n:::\n\n")
                                && ((!t && self.unordered_list.is_some())
                                    || (t && self.ordered_list.is_some()))
                            {
                                if !block.ends_with("\n\n") {
                                    writeln!(block)?;
                                }
                                writeln!(block, ":::\n")?;
                            }
                            block = self.process_block(block, &mut blocks);
                        } else if !block.ends_with("\n\n") {
                            writeln!(block)?;
                        }
                        let _ = lists.pop().unwrap();
                        let _ = indents.pop().unwrap();
                    }
                    pd::TagEnd::Item => {
                        depth -= 1;
                        if block.ends_with(":::") {
                            block.insert(block.len() - 3, '\n');
                            writeln!(block, "\n")?;
                        } else if !block.ends_with("\n\n") {
                            writeln!(block)?;
                        }
                    }
                    pd::TagEnd::BlockQuote(_) => {
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
                    pd::TagEnd::Table => {
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
                    pd::TagEnd::CodeBlock => {
                        depth -= 1;
                        if depth == 0 {
                            if let Some(info) = info.take() {
                                if info == "[ignore]" {
                                    writeln!(block, "{}", source.replacen("[ignore]", "", 1))?;
                                } else if let Some(style) = &self.fenced_code_block {
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
                            } else {
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
                    }
                    _ => {}
                }, // end end tags

                // All other events
                _ => {
                    if !disabled && img_link_depth == 0 {
                        write!(block, "{source}")?;
                    }
                }
            } // end match event
        } // end iterating events

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
            resolve_double_style(outer.as_ref(), inner.as_ref(), double.as_ref(), &mut block);
        }
        if self.debug {
            eprintln!("---\nblock = {block:?}");
        }
        blocks.push(block);
        String::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions

/**
Replace "double style" spans with a single style
*/
fn resolve_double_style(
    outer: Option<&String>,
    inner: Option<&String>,
    double: Option<&String>,
    block: &mut String,
) {
    if let Some(style_outer) = outer
        && let Some(style_inner) = inner
        && let Some(style_double) = double
    {
        // All three styles must be defined...

        let replace =
            format!("]{{custom-style=\"{style_inner}\"}}]{{custom-style=\"{style_outer}\"}}");
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
