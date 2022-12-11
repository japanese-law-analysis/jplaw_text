use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct ParenText {
  position: usize,
  char: char,
}

/// 一つの段落を入れることで以下の情報とそれらが出現する位置のリストを生成する
/// - 括弧の役割とその深さ（「開き括弧ではないただの文字」という情報も含む）
/// - 法律名や条文数などの表記とその種類
pub async fn get_paren_annotation(text: &str) -> Vec<ParenText> {
  let mut counter = 0;
  let chars = text.chars();
  let mut chars = tokio_stream::iter(chars);
  let mut paren_info_lst = Vec::new();
  let mut paren_depth = 0;
  while let Some(c) = chars.next().await {
    counter += 1;
    match c {
      '「' => {
        paren_depth += 1;
        paren_info_lst.push((counter, c, paren_depth));
      }
      '」' => {
        paren_info_lst.push((counter, c, paren_depth));
        paren_depth -= 1;
      }
      '（' => {
        paren_depth += 1;
        paren_info_lst.push((counter, c, paren_depth));
      }
      '）' => {
        paren_info_lst.push((counter, c, paren_depth));
        paren_depth -= 1;
      }
      _ => (),
    }
  }
  println!("paren_info_lst: {paren_info_lst:?}");
  split_lst_and_remove_text(&paren_info_lst)
}

/// 再帰関数
/// 深さがフラットになる地点で括弧の列を分割し、
/// - 要素の先頭が閉じ括弧
/// - 要素の最後が開き括弧
/// の場合にそれらを除去し、再度分割する
/// 要素数が0か1になったときに終了
fn split_lst_and_remove_text(lst: &Vec<(usize, char, isize)>) -> Vec<ParenText> {
  let size = lst.len();
  if size == 0 {
    Vec::new()
  } else if size == 1 {
    let d = ParenText {
      position: lst[0].0,
      char: lst[0].1,
    };
    vec![d]
  } else {
    let mut v = Vec::new();
    let mut is_remove_head = false;
    let mut is_remove_last = false;
    if lst[0].1 == '）' || lst[0].1 == '」' {
      let d = ParenText {
        position: lst[0].0,
        char: lst[0].1,
      };
      v.push(d);
      is_remove_head = true;
    }
    if lst[size - 1].1 == '（' || lst[size - 1].1 == '「' {
      let d = ParenText {
        position: lst[size - 1].0,
        char: lst[size - 1].1,
      };
      v.push(d);
      is_remove_last = true;
    }
    let range = match (is_remove_head, is_remove_last) {
      (true, true) => 1..size - 1,
      (true, false) => 1..size,
      (false, true) => 0..size - 1,
      (false, false) => 0..size,
    };
    let children = range.map(|i| lst[i]).collect::<Vec<_>>();
    let head_depth = children[0].2;
    let children = children
      .iter()
      .map(|(position, c, depth)| (position, c, depth + (0 - head_depth) + 1))
      .collect::<Vec<_>>();
    println!("children : {children:?}");
    let mut children_2 = children.clone();
    children_2.reverse();
    let lst_len = children.len();
    let gap = children[lst_len - 1].2 - children[0].2;
    let mut children_text_lst = if gap == 0 {
      Vec::new()
    } else {
      println!("gap : {gap}");
      let children_stream = children_2
        .iter()
        .map(|(position, c, depth)| (position, c, depth - gap));
      let mut lst = Vec::new();
      let mut tmp = Vec::new();
      let mut count = 0;
      for (position, c, depth) in children_stream {
        if depth == count {
          tmp.push((position, c, depth));
          tmp.reverse();
          lst.push((count, tmp.clone()));
          count -= 1;
          tmp = Vec::new();
        } else {
          tmp.push((position, c, depth))
        }
      }
      tmp.reverse();
      lst.push((count, tmp.clone()));
      // 深さがリセットされる地点で区切っている
      lst.reverse();
      println!("children lst : {lst:?}");
      lst
        .iter()
        .map(|(gap, lst)| {
          lst
            .iter()
            .map(|(position, c, depth)| (***position, ***c, depth + (0 - *gap) + 1))
            .collect::<Vec<_>>()
        })
        .map(|lst| {
          let size = lst.len();
          if size == 0 {
            Vec::new()
          } else {
            let mut is_remove = false;
            if (lst[0].1 == '（' && lst[size - 1].1 == '）')
              || (lst[0].1 == '「' && lst[size - 1].1 == '」')
            {
              is_remove = true;
            }
            let range = if is_remove { 1..size - 1 } else { 0..size };
            let lst = range.map(|i| lst[i]).collect::<Vec<_>>();
            split_lst_and_remove_text(&lst)
          }
        })
        .collect::<Vec<_>>()
        .concat()
    };
    println!("{v:?}");
    v.append(&mut children_text_lst);
    v
  }
}

#[tokio::test]
async fn check1() {
  let text = "〜「〜」〜";
  let expect: Vec<ParenText> = Vec::new();
  let gen = get_paren_annotation(text).await;
  assert_eq!(expect, gen);
}

#[tokio::test]
async fn check2() {
  let text = "〜「〜「〜」〜「」〜「〜「〜」〜「〜」〜";
  let expect: Vec<ParenText> = vec![
    ParenText {
      position: 8,
      char: '「',
    },
    ParenText {
      position: 17,
      char: '「',
    },
  ];
  let gen = get_paren_annotation(text).await;
  assert_eq!(expect, gen);
}


#[tokio::test]
async fn check3() {
  let text = "〜「〜「〜」〜」」〜「〜「〜」〜」〜」〜";
  let expect: Vec<ParenText> = vec![
    ParenText {
      position: 8,
      char: '」',
    },
    ParenText {
      position: 17,
      char: '」',
    },
  ];
  let gen = get_paren_annotation(text).await;
  assert_eq!(expect, gen);
}
