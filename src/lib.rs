//!
//! [e-gov 法令検索](https://elaws.e-gov.go.jp/)のXMLデータから特定の条項の中身を取得するライブラリを提供する
//!
//! ---
//! [MIT License](https://github.com/japanese-law-analysis/listup_law/blob/master/LICENSE)
//! (c) 2023 Naoki Kaneko (a.k.a. "puripuri2100")
//!

use encoding_rs::UTF_8;
use quick_xml::{encoding, events::Event, Reader};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
  pub article_info: Article,
  pub contents: LawContents,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Article {
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
  pub sub_item: Option<Vec<String>>,
  /// 附則の場合
  #[serde(skip_serializing_if = "Option::is_none")]
  pub suppl_provision_title: Option<String>,
}

impl Article {
  fn new() -> Self {
    Article {
      article: String::new(),
      paragraph: None,
      item: None,
      sub_item: None,
      suppl_provision_title: None,
    }
  }

  fn update_article(&mut self, article: String) {
    *self = Article {
      article,
      paragraph: None,
      item: None,
      sub_item: None,
      suppl_provision_title: self.clone().suppl_provision_title,
    }
  }

  fn update_paragraph(&mut self, p: String) {
    *self = Article {
      article: self.clone().article,
      paragraph: Some(p),
      item: None,
      sub_item: None,
      suppl_provision_title: self.clone().suppl_provision_title,
    }
  }

  fn update_item(&mut self, i: String) {
    *self = Article {
      article: self.clone().article,
      paragraph: self.clone().paragraph,
      item: Some(i),
      sub_item: None,
      suppl_provision_title: self.clone().suppl_provision_title,
    }
  }

  fn update_sub_item(&mut self, n: usize, s: String) {
    let mut new_sub_item_lst = Vec::new();
    if let Some(sub_item_lst) = &self.sub_item {
      for i in 1..=n {
        if i == n {
          new_sub_item_lst.push(s.clone())
        } else {
          match sub_item_lst.get(i) {
            None => new_sub_item_lst.push(String::new()),
            Some(s) => new_sub_item_lst.push(s.clone()),
          }
        }
      }
    } else {
      for i in 1..=n {
        if i == n {
          new_sub_item_lst.push(s.clone())
        } else {
          new_sub_item_lst.push(String::new())
        }
      }
    };

    *self = Article {
      article: self.clone().article,
      paragraph: self.clone().paragraph,
      item: self.clone().item,
      sub_item: Some(new_sub_item_lst),
      suppl_provision_title: self.clone().suppl_provision_title,
    }
  }

  fn update_suppl_provision_title(&mut self, title: String) {
    *self = Article {
      article: String::new(),
      paragraph: None,
      item: None,
      sub_item: None,
      suppl_provision_title: Some(title),
    }
  }
}

pub async fn xml_to_law_text(xml_buf: &[u8]) -> Result<Vec<LawText>, SearchArticleError> {
  let mut buf = Vec::new();
  let mut xml_reader = Reader::from_reader(xml_buf);
  xml_reader.trim_text(true);

  let mut law_text_lst = vec![];

  let mut now_article = Article::new();

  let mut is_ruby_rt = false;

  let mut is_sentence = false;

  let mut tmp_text = String::new();

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
          now_article.update_article(article_num_str);
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
          now_article.update_paragraph(num_str);
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
          now_article.update_item(num_str);
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
          now_article.update_sub_item(1, num_str);
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
          now_article.update_sub_item(2, num_str);
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
          now_article.update_sub_item(3, num_str);
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
          now_article.update_sub_item(4, num_str);
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
          now_article.update_sub_item(5, num_str);
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
          now_article.update_sub_item(6, num_str);
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
          now_article.update_sub_item(7, num_str);
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
          now_article.update_sub_item(8, num_str);
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
          now_article.update_sub_item(9, num_str);
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
          now_article.update_suppl_provision_title(suppl_provision_title_str);
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
        }
        _ => (),
      },
      Ok(Event::End(tag)) => match tag.name().as_ref() {
        b"Rt" => is_ruby_rt = false,
        b"Sentence" => is_sentence = false,
        b"ParagraphSentence" | b"ItemSentence" | b"Subitem1Sentence" | b"Subitem2Sentence"
        | b"Subitem3Sentence" | b"Subitem4Sentence" | b"Subitem5Sentence" | b"Subitem6Sentence"
        | b"Subitem7Sentence" | b"Subitem8Sentence" | b"Subitem9Sentence" => {
          if !tmp_text.is_empty() {
            let law_text = LawText {
              article_info: now_article.clone(),
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
        }
        b"TableRow" => {
          if !tmp_table_col.is_empty() {
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
              article_info: now_article.clone(),
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
        if is_sentence && !is_ruby_rt {
          let text_str = encoding::decode(&text.into_inner(), UTF_8)
            .unwrap()
            .trim()
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

pub async fn search_law_text(
  xml_buf: &[u8],
  target: &Article,
) -> Result<Vec<LawText>, SearchArticleError> {
  let v = xml_to_law_text(xml_buf)
    .await?
    .iter()
    .filter(|v| {
      let article_info = &v.article_info;
      let is_t_a = article_info.article == target.article;
      // targetのparagraphがNoneならば、article_infoのそれがどんな値でも良い
      // どうせ全ての値がtrueでないといけないので、目標の物ではない場合は他の値によって弾ける
      let is_t_p = target.paragraph.is_none() || article_info.paragraph == target.paragraph;
      let is_t_i = target.item.is_none() || article_info.item == target.item;
      let is_t_si = target.sub_item.is_none()
        || target
          .clone()
          .sub_item
          .map(|lst| {
            let len = lst.len();
            match &article_info.sub_item {
              None => false,
              Some(v) => {
                let lst2 = v.iter().take(len).cloned().collect::<Vec<_>>();
                lst2 == lst
              }
            }
          })
          .unwrap_or(false);
      let is_t_spt = target.suppl_provision_title.is_none()
        || article_info.suppl_provision_title == target.suppl_provision_title;
      is_t_a && is_t_p && is_t_i && is_t_si && is_t_spt
    })
    .cloned()
    .collect::<Vec<_>>();
  Ok(v)
}
