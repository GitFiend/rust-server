use std::fs::read_to_string;
use std::path::Path;

use crate::parser::standard_parsers::{ANY_WORD, UNTIL_LINE_END};
use crate::parser::{parse_all_err, Parser};
use crate::server::request_util::R;
use crate::{and, character, many, map2, or, word};

pub fn load_packed_refs(repo_path: &str) -> R<Vec<String>> {
  let path = Path::new(repo_path).join(".git").join("packed-refs");

  let text = read_to_string(path).map_err(|e| e.to_string())?;

  let lines = parse_all_err(P_LINES, &text);

  lines
}

#[derive(Debug)]
enum PRLine {
  Ref(String),
  Other,
}

const P_LOCAL: Parser<PRLine> = map2!(
  and!(
    ANY_WORD,
    character!(' '),
    word!("refs/heads/"),
    UNTIL_LINE_END
  ),
  res,
  PRLine::Ref(res.3)
);

const P_REMOTE: Parser<PRLine> = map2!(
  and!(
    ANY_WORD,
    character!(' '),
    word!("refs/remotes/"),
    UNTIL_LINE_END
  ),
  res,
  PRLine::Ref(remove_remote(res.3))
);

fn remove_remote(ref_part: String) -> String {
  if let Some((_, tail)) = ref_part.split_once('/') {
    return tail.to_string();
  }
  ref_part
}

const P_OTHER: Parser<PRLine> = map2!(UNTIL_LINE_END, __, PRLine::Other);

const P_LINE: Parser<PRLine> = or!(P_LOCAL, P_REMOTE, P_OTHER);

const P_LINES: Parser<Vec<String>> = map2!(
  many!(P_LINE),
  lines,
  lines
    .into_iter()
    .flat_map(|l| match l {
      PRLine::Ref(line) => Some(line),
      PRLine::Other => None,
    })
    .collect()
);
