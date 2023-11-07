use console::{style, Style};
use similar::{ChangeTag, TextDiff};

use std::fmt::{Write as _, self};
use std::io::Write;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

struct Line(Option<usize>);

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self.0 {
          None => write!(f, "    "),
          Some(idx) => write!(f, "{:<4}", idx + 1),
      }
  }
}

pub fn process_error_output(result: anyhow::Result<()>) -> anyhow::Result<()> {
    if let Err(e) = result {
        let stderr = std::io::stderr();
        let mut stderr = stderr.lock();
        // atty是一个库，用于检测终端输入输出环境
        if atty::is(atty::Stream::Stderr) {
            let red = Style::new().red();
            write!(stderr, "{}", red.apply_to(format!("{:?}", e)))?;
        } else {
            writeln!(stderr, "{:?}", e)?;
        }
    }
    
    Ok(())
}

pub fn diff_text(text1: &str, text2: &str) -> anyhow::Result<(String, String, String)> {
    let mut output = String::new();
    let mut output1 = String::new();
    let mut output2 = String::new();
    let diff = TextDiff::from_lines(text1, text2);
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            writeln!(&mut output, "{:-^1$}", "-", 80)?;
            writeln!(&mut output1, "{:-^1$}", "-", 80)?;
            writeln!(&mut output2, "{:-^1$}", "-", 80)?;
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };
                write!(
                    &mut output,
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                )?;
                match change.tag() {
                    ChangeTag::Delete => {
                      write!(
                          &mut output1,
                          "{} |{}",
                          style(Line(change.old_index())).dim(),
                          s.apply_to(sign).bold(),
                      )?;
                    },
                    ChangeTag::Insert => {
                      write!(
                          &mut output2,
                          "{} |{}",
                          style(Line(change.new_index())).dim(),
                          s.apply_to(sign).bold(),
                      )?;
                    },
                    ChangeTag::Equal => {
                      write!(
                          &mut output1,
                          "{} |{}",
                          style(Line(change.old_index())).dim(),
                          s.apply_to(sign).bold(),
                      )?;
                      write!(
                        &mut output2,
                        "{} |{}",
                        style(Line(change.new_index())).dim(),
                        s.apply_to(sign).bold(),
                    )?;
                    },
                };
                
               
                for (emphasized, value) in change.iter_strings_lossy() {
                    match change.tag() {
                        ChangeTag::Delete => {
                          write!(&mut output1, "{}", s.apply_to(value.clone()))?;
                        },
                        ChangeTag::Insert => {
                          write!(&mut output2, "{}", s.apply_to(value.clone()))?;
                        },
                        ChangeTag::Equal => {
                          write!(&mut output1, "{}", s.apply_to(value.clone()))?;
                          write!(&mut output2, "{}", s.apply_to(value.clone()))?;
                        },
                    };
                    
                    if emphasized {
                        write!(&mut output, "{}", s.apply_to(value).underlined().on_black())?;
                    } else {
                        write!(&mut output, "{}", s.apply_to(value))?;
                    }
                }
                if change.missing_newline() {
                    writeln!(&mut output)?;
                    writeln!(&mut output1)?;
                    writeln!(&mut output2)?;
                }
            }
        }
    }
    Ok((output, output1, output2))
}

// 高亮展示文字
pub fn highlight_text(text: &str, extension: &str, theme: Option<&str>) -> anyhow::Result<String> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts: ThemeSet = ThemeSet::load_defaults();

    let syntax = if let Some(s) = ps.find_syntax_by_extension(extension) {
        s
    } else {
        ps.find_syntax_plain_text()
    };
    let mut h = HighlightLines::new(syntax, &ts.themes[theme.unwrap_or("base16-ocean.dark")]);
    let mut output = String::new();
    for line in LinesWithEndings::from(text) {
        let ranges = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        write!(&mut output, "{}", escaped)?;
    }

    Ok(output)
}