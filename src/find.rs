use encoding_rs::UTF_8;
use quick_xml::{encoding, events::Event, Reader};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::AsyncBufRead;

#[derive(Debug, Error, Clone, PartialEq, Eq, Hash)]
pub enum SearchArticleError {
  #[error("not found article number")]
  NotFoundArticleNumber,
  #[error("xml parser error")]
  XmlParserError,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum LawTableContents {
  Text(String),
  // Link { row: usize, column: usize },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LawTableColumn {
  pub rowspan: usize,
  pub colspan: usize,
  pub contents: LawTableContents,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LawTable {
  pub row: Vec<LawTableColumn>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum LawContents {
  Text(String),
  Table(Vec<LawTable>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct LawText {
  pub is_child: bool,
  pub contents: LawContents,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, Default)]
pub struct ArticleTargetInfo {
  /// 条
  pub article: String,
  /// 項
  #[serde(skip_serializing_if = "Option::is_none")]
  pub paragraph: Option<String>,
  /// 号
  #[serde(skip_serializing_if = "Option::is_none")]
  pub item: Option<String>,
  /// イロハなど（深さも必要）
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sub_item: Option<(usize, String)>,
  /// 附則の場合
  #[serde(skip_serializing_if = "Option::is_none")]
  pub suppl_provision_title: Option<String>,
}

pub async fn search_law_text<T>(
  xml_reader: &mut Reader<T>,
  target: &ArticleTargetInfo,
) -> Result<Vec<LawText>, SearchArticleError>
where
  T: AsyncBufRead + Unpin,
{
  let mut buf = Vec::new();
  xml_reader.trim_text(true);

  let mut law_text_lst = vec![];

  let mut is_target_article = false;
  let mut is_target_paragraph = target.paragraph.is_none();
  let mut is_target_item = target.item.is_none();
  let mut is_target_sub_item_1 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 1)
    .unwrap_or(true);
  let mut is_target_sub_item_2 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 2)
    .unwrap_or(true);
  let mut is_target_sub_item_3 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 3)
    .unwrap_or(true);
  let mut is_target_sub_item_4 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 4)
    .unwrap_or(true);
  let mut is_target_sub_item_5 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 5)
    .unwrap_or(true);
  let mut is_target_sub_item_6 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 6)
    .unwrap_or(true);
  let mut is_target_sub_item_7 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 7)
    .unwrap_or(true);
  let mut is_target_sub_item_8 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 8)
    .unwrap_or(true);
  let mut is_target_sub_item_9 = target
    .sub_item
    .as_ref()
    .map(|(i, _)| *i < 9)
    .unwrap_or(true);
  let mut is_target_suppl_provision = target.suppl_provision_title.is_none();

  let mut is_ruby_rt = false;

  let mut is_sentence = false;

  let mut is_child = false;

  let mut tmp_text = String::new();

  let mut is_table_column = false;
  let mut tmp_table_row = Vec::new();
  let mut tmp_table_col = Vec::new();
  let mut tmp_rowspan = 1;
  let mut tmp_colspan = 1;

  loop {
    match xml_reader.read_event_into_async(&mut buf).await {
      Ok(Event::Start(tag)) => match tag.name().as_ref() {
        b"Article" => {
          let article_num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_article = article_num_str == target.article;
          is_child = false;
        }
        b"Paragraph" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_paragraph = target
            .paragraph
            .as_ref()
            .map(|s| s == &num_str)
            .unwrap_or(true);
          is_child = target.paragraph.is_none();
        }
        b"Item" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_item = target.item.as_ref().map(|s| s == &num_str).unwrap_or(true);
          is_child = target.item.is_none();
        }
        b"Subitem1" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_1 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 1 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 1)
            .unwrap_or(true);
        }
        b"Subitem2" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_2 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 2 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 2)
            .unwrap_or(true);
        }
        b"Subitem3" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_3 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 3 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 3)
            .unwrap_or(true);
        }
        b"Subitem4" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_4 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 4 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 4)
            .unwrap_or(true);
        }
        b"Subitem5" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_5 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 5 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 6)
            .unwrap_or(true);
        }
        b"Subitem6" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_6 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 6 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 6)
            .unwrap_or(true);
        }
        b"Subitem7" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_7 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 7 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 7)
            .unwrap_or(true);
        }
        b"Subitem8" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_8 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 8 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 8)
            .unwrap_or(true);
        }
        b"Subitem9" => {
          let num_str = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "Num")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap();
          is_target_sub_item_9 = target
            .sub_item
            .as_ref()
            .map(|(i, s)| *i == 8 && s == &num_str)
            .unwrap_or(true);
          is_child = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 9)
            .unwrap_or(true);
        }
        b"SupplProvision" => {
          let suppl_provision_title_str = tag
            .attributes()
            .find(|res| {
              encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "AmendLawNum"
            })
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .to_string()
            })
            .unwrap_or_default();
          is_target_suppl_provision =
            Some(suppl_provision_title_str) == target.suppl_provision_title;
        }
        b"Sentence" => {
          is_sentence = true;
        }
        b"Rt" => is_ruby_rt = true,
        b"TableColumn" => {
          let row_span = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "rowspan")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .parse::<usize>()
                .unwrap()
            })
            .unwrap_or(1);
          tmp_rowspan = row_span;
          let col_span = tag
            .attributes()
            .find(|res| encoding::decode(res.as_ref().unwrap().key.0, UTF_8).unwrap() == "colspan")
            .map(|res| {
              encoding::decode(&res.unwrap().value, UTF_8)
                .unwrap()
                .parse::<usize>()
                .unwrap()
            })
            .unwrap_or(1);
          tmp_colspan = col_span;
          is_table_column = true;
        }
        _ => (),
      },
      Ok(Event::End(tag)) => match tag.name().as_ref() {
        b"Article" => {
          is_target_article = false;
          is_child = false;
        }
        b"Paragraph" => {
          is_target_paragraph = target.paragraph.is_none();
          is_child = false;
        }
        b"Item" => {
          is_target_item = target.item.is_none();
          is_child = false;
        }
        b"Subitem1" => {
          is_target_sub_item_1 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 1)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem2" => {
          is_target_sub_item_2 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 2)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem3" => {
          is_target_sub_item_3 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 3)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem4" => {
          is_target_sub_item_4 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 4)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem5" => {
          is_target_sub_item_5 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 5)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem6" => {
          is_target_sub_item_6 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 6)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem7" => {
          is_target_sub_item_7 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 7)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem8" => {
          is_target_sub_item_8 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 8)
            .unwrap_or(true);
          is_child = false;
        }
        b"Subitem9" => {
          is_target_sub_item_9 = target
            .sub_item
            .as_ref()
            .map(|(i, _)| *i < 9)
            .unwrap_or(true);
          is_child = false;
        }
        b"SupplProvision" => is_target_suppl_provision = target.suppl_provision_title.is_none(),
        b"Rt" => is_ruby_rt = false,
        b"Sentence" => is_sentence = false,
        b"ParagraphSentence" | b"ItemSentence" | b"Subitem1Sentence" | b"Subitem2Sentence"
        | b"Subitem3Sentence" | b"Subitem4Sentence" | b"Subitem5Sentence" | b"Subitem6Sentence"
        | b"Subitem7Sentence" | b"Subitem8Sentence" | b"Subitem9Sentence" => {
          if !tmp_text.is_empty() {
            let law_text = LawText {
              is_child,
              contents: LawContents::Text(tmp_text),
            };
            law_text_lst.push(law_text);
            tmp_text = String::new();
          }
        }
        b"TableColumn" => {
          if !tmp_text.is_empty() {
            let law_column = LawTableColumn {
              rowspan: tmp_rowspan,
              colspan: tmp_colspan,
              contents: LawTableContents::Text(tmp_text),
            };
            tmp_table_col.push(law_column);
          }
          tmp_rowspan = 1;
          tmp_colspan = 1;
          tmp_text = String::new();
          is_table_column = false;
        }
        b"TableRow" => {
          if !tmp_text.is_empty() {
            let row = LawTable {
              row: tmp_table_col.clone(),
            };
            tmp_table_row.push(row);
          }
          tmp_table_col = Vec::new();
          tmp_rowspan = 1;
          tmp_colspan = 1;
          tmp_text = String::new();
        }
        b"Table" => {
          if !tmp_table_row.is_empty() {
            let law_text = LawText {
              is_child,
              contents: LawContents::Table(tmp_table_row.clone()),
            };
            law_text_lst.push(law_text);
          }
          tmp_table_row = Vec::new();
          tmp_table_col = Vec::new();
          tmp_rowspan = 1;
          tmp_colspan = 1;
          tmp_text = String::new();
        }
        _ => (),
      },
      Ok(Event::Text(text)) => {
        if is_target_article
          && ((is_target_paragraph
            && is_target_item
            && is_target_sub_item_1
            && is_target_sub_item_2
            && is_target_sub_item_3
            && is_target_sub_item_4
            && is_target_sub_item_5
            && is_target_sub_item_6
            && is_target_sub_item_7
            && is_target_sub_item_8
            && is_target_sub_item_9
            && is_target_suppl_provision
            && is_sentence
            && !is_ruby_rt)
            || is_table_column)
        {
          let text_str = encoding::decode(&text.into_inner(), UTF_8)
            .unwrap()
            .to_string();
          tmp_text.push_str(&text_str);
        }
      }
      Ok(Event::Eof) => break,
      Err(_) => return Err(SearchArticleError::XmlParserError),
      _ => (),
    }
  }
  Ok(law_text_lst)
}
