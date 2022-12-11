#[tokio::main]
async fn main() {
  let text = "〜「〜「〜」〜」」〜「〜「〜」〜」〜」〜";
  let lst = jplaw_text::parse::get_paren_annotation(text).await;
  println!("result: {lst:?}")
}
