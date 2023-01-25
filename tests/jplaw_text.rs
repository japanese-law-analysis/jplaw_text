use jplaw_text::*;
use quick_xml::Reader;
use tokio::{self, io::BufReader};

const LAW_XML: &str = r#"
<Section Num="4">
<SectionTitle>第四節　住所</SectionTitle>
<Article Num="28">
<ArticleCaption>（管理人の権限）</ArticleCaption>
<ArticleTitle>第二十八条</ArticleTitle>
<Paragraph Num="1">
  <ParagraphNum/>
  <ParagraphSentence>
    <Sentence Num="1">管理人は、第百三条に規定する権限を超える行為を必要とするときは、家庭裁判所の許可を得て、その行為をすることができる。</Sentence>
    <Sentence Num="2">不在者の生死が明らかでない場合において、その管理人が不在者が定めた権限を超える行為を必要とするときも、同様とする。</Sentence>
  </ParagraphSentence>
</Paragraph>
</Article>
<Article Num="29">
<ArticleCaption>（管理人の担保提供及び報酬）</ArticleCaption>
<ArticleTitle>第二十九条</ArticleTitle>
<Paragraph Num="1">
  <ParagraphNum/>
  <ParagraphSentence>
    <Sentence>家庭裁判所は、管理人に財産の管理及び返還について相当の担保を立てさせることができる。</Sentence>
  </ParagraphSentence>
</Paragraph>
<Paragraph Num="2">
  <ParagraphNum>２</ParagraphNum>
  <ParagraphSentence>
    <Sentence>家庭裁判所は、管理人と不在者との関係その他の事情により、不在者の財産の中から、相当な報酬を管理人に与えることができる。</Sentence>
  </ParagraphSentence>
</Paragraph>
</Article>
<Article Num="30">
<ArticleCaption>（失<Ruby>踪<Rt>そう</Rt>
</Ruby>の宣告）</ArticleCaption>
<ArticleTitle>第三十条</ArticleTitle>
<Paragraph Num="1">
  <ParagraphNum/>
  <ParagraphSentence>
    <Sentence>不在者の生死が七年間明らかでないときは、家庭裁判所は、利害関係人の請求により、失<Ruby>踪<Rt>そう</Rt>
</Ruby>の宣告をすることができる。</Sentence>
  </ParagraphSentence>
</Paragraph>
<Paragraph Num="2">
  <ParagraphNum>２</ParagraphNum>
  <ParagraphSentence>
    <Sentence>戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が<Ruby>止<Rt>や</Rt>
</Ruby>んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。</Sentence>
  </ParagraphSentence>
</Paragraph>
</Article>
</Section>
"#;

#[tokio::test]
async fn check1() {
  let target = ArticleTargetInfo {
    article: "30".to_string(),
    paragraph: Some("2".to_string()),
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      is_child : false,
      contents : LawContents::Text("戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が止んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。".to_string())
    },
  ];
  let gen_law_text_lst = search_law_text(LAW_XML, &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}

#[tokio::test]
async fn check2() {
  let target = ArticleTargetInfo {
    article: "30".to_string(),
    paragraph: None,
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      is_child : true,
      contents : LawContents::Text("不在者の生死が七年間明らかでないときは、家庭裁判所は、利害関係人の請求により、失踪の宣告をすることができる。".to_string())
    },
    LawText {
      is_child : true,
      contents : LawContents::Text("戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が止んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。".to_string())
    },
  ];
  let gen_law_text_lst = search_law_text(LAW_XML, &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}

#[tokio::test]
async fn check3() {
  let str = r#"
<Chapter Delete="false" Hide="false" Num="5">
  <ChapterTitle>第五章　届出料金の算定</ChapterTitle>
  <Article Delete="false" Hide="false" Num="30">
    <ArticleCaption>（届出料金に関する準用）</ArticleCaption>
    <ArticleTitle>第三十条</ArticleTitle>
    <Paragraph Hide="false" Num="1" OldStyle="false">
      <ParagraphNum/>
      <ParagraphSentence>
        <Sentence Num="1" WritingMode="vertical">第三条から第二十六条まで及び第二十七条から前条までの規定は、法第十八条第四項の規定により託送供給等約款で設定した料金を変更しようとする一般送配電事業者が、変更しようとする託送供給等約款で設定する料金を算定する場合に準用する。</Sentence>
        <Sentence Num="2" WritingMode="vertical">この場合において、次の表の上欄に掲げる規定中同表の中欄に掲げる字句は、それぞれ同表の下欄に掲げる字句に読み替えるものとする。</Sentence>
      </ParagraphSentence>
      <TableStruct>
        <Table WritingMode="vertical">
          <TableRow>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">第三条 </Sentence>
            </TableColumn>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">原価等</Sentence>
            </TableColumn>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">届出原価等</Sentence>
            </TableColumn>
          </TableRow>
          <TableRow>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">第四条第三項</Sentence>
            </TableColumn>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">様式第一第一表及び様式第二第一表</Sentence>
            </TableColumn>
            <TableColumn BorderBottom="solid" BorderLeft="solid" BorderRight="solid" BorderTop="solid">
              <Sentence WritingMode="vertical">様式第一第一表</Sentence>
            </TableColumn>
          </TableRow>
        </Table>
      </TableStruct>
    </Paragraph>
  </Article>
</Chapter>"#;

  let target = ArticleTargetInfo {
    article: "30".to_string(),
    paragraph: None,
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      is_child : false,
      contents : LawContents::Text("第三条から第二十六条まで及び第二十七条から前条までの規定は、法第十八条第四項の規定により託送供給等約款で設定した料金を変更しようとする一般送配電事業者が、変更しようとする託送供給等約款で設定する料金を算定する場合に準用する。この場合において、次の表の上欄に掲げる規定中同表の中欄に掲げる字句は、それぞれ同表の下欄に掲げる字句に読み替えるものとする。".to_string())
    },
    LawText {
    is_child: false,
    contents: LawContents::Table(vec![
      LawTable {
        row: vec![
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("第三条 ".to_string()),
          },
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("原価等".to_string()),
          },
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("届出原価等".to_string()),
          },
        ],
      },
      LawTable {
        row: vec![
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("第四条第三項".to_string()),
          },
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("様式第一第一表及び様式第二第一表".to_string()),
          },
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("様式第一第一表".to_string()),
          },
        ],
      },
    ]),
  }];
  let gen_law_text_lst = search_law_text(str, &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}
