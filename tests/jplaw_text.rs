use jplaw_text::*;
use tokio;

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
async fn get_all_info() {
  let law_text_lst = vec![
    LawText {
      article_info: Article {
        article: "28".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("管理人は、第百三条に規定する権限を超える行為を必要とするときは、家庭裁判所の許可を得て、その行為をすることができる。不在者の生死が明らかでない場合において、その管理人が不在者が定めた権限を超える行為を必要とするときも、同様とする。".to_string())
    },
    LawText {
      article_info: Article {
        article: "29".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("家庭裁判所は、管理人に財産の管理及び返還について相当の担保を立てさせることができる。".to_string())
    },
    LawText {
      article_info: Article {
        article: "29".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("家庭裁判所は、管理人と不在者との関係その他の事情により、不在者の財産の中から、相当な報酬を管理人に与えることができる。".to_string())
    },
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("不在者の生死が七年間明らかでないときは、家庭裁判所は、利害関係人の請求により、失踪の宣告をすることができる。".to_string())
    },
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が止んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。".to_string())
    },
  ];
  let gen_law_text_lst = xml_to_law_text(LAW_XML.as_bytes()).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}

#[tokio::test]
async fn check1() {
  let target = Article {
    article: "30".to_string(),
    paragraph: Some("2".to_string()),
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が止んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。".to_string())
    },
  ];
  let gen_law_text_lst = search_law_text(LAW_XML.as_bytes(), &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}

#[tokio::test]
async fn check2() {
  let target = Article {
    article: "30".to_string(),
    paragraph: None,
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("不在者の生死が七年間明らかでないときは、家庭裁判所は、利害関係人の請求により、失踪の宣告をすることができる。".to_string())
    },
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("戦地に臨んだ者、沈没した船舶の中に在った者その他死亡の原因となるべき危難に遭遇した者の生死が、それぞれ、戦争が止んだ後、船舶が沈没した後又はその他の危難が去った後一年間明らかでないときも、前項と同様とする。".to_string())
    },
  ];
  let gen_law_text_lst = search_law_text(LAW_XML.as_bytes(), &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}

const LAW_XML_2: &str = r#"
<Section>
<Article Num="31">
<ArticleCaption>（安定供給確保支援法人の指定及び業務）</ArticleCaption>
<ArticleTitle>第三十一条</ArticleTitle>
  <Paragraph Num="1">
    <ParagraphNum/>
    <ParagraphSentence>
      <Sentence Num="1" WritingMode="vertical">主務大臣は、安定供給確保基本指針及び安定供給確保取組方針に基づき、主務省令で定めるところにより、一般社団法人、一般財団法人その他主務省令で定める法人であって、第三項に規定する業務（以下この章及び第九十六条第三号において「安定供給確保支援業務」という。）に関し次の各号のいずれにも適合すると認められるものを、その申請により、特定重要物資ごとに安定供給確保支援法人として指定することができる。</Sentence>
    </ParagraphSentence>
    <Item Num="1">
      <ItemTitle>一</ItemTitle>
      <ItemSentence>
        <Sentence Num="1" WritingMode="vertical">安定供給確保支援業務を適正かつ確実に実施することができる経理的基礎及び技術的能力を有するものであること。</Sentence>
      </ItemSentence>
    </Item>
    <Item Num="2">
      <ItemTitle>二</ItemTitle>
      <ItemSentence>
        <Sentence Num="1" WritingMode="vertical">安定供給確保支援業務の実施体制が安定供給確保基本指針に照らし適切であること。</Sentence>
      </ItemSentence>
    </Item>
  </Paragraph>
  <Paragraph Num="2">
    <ParagraphNum>２</ParagraphNum>
    <ParagraphSentence>
      <Sentence Num="1" WritingMode="vertical">次の各号のいずれかに該当する者は、前項の規定による指定（以下この節において「指定」という。）を受けることができない。</Sentence>
    </ParagraphSentence>
    <Item Num="1">
      <ItemTitle>一</ItemTitle>
      <ItemSentence>
        <Sentence Num="1" WritingMode="vertical">この法律の規定に違反し、刑に処せられ、その執行を終わり、又は執行を受けることがなくなった日から起算して二年を経過しない者</Sentence>
      </ItemSentence>
    </Item>
    <Item Num="2">
      <ItemTitle>二</ItemTitle>
      <ItemSentence>
        <Sentence Num="1" WritingMode="vertical">第四十一条第一項又は第二項の規定により指定を取り消され、その取消しの日から起算して二年を経過しない者</Sentence>
      </ItemSentence>
    </Item>
  </Paragraph>
</Article>
<Article Num="32">
  <ArticleCaption>（安定供給確保支援法人の指定の公示等）</ArticleCaption>
  <ArticleTitle>第三十二条</ArticleTitle>
  <Paragraph Num="1">
    <ParagraphNum/>
    <ParagraphSentence>
      <Sentence Num="1" WritingMode="vertical">主務大臣は、指定をしたときは、当該指定に係る安定供給確保支援法人の名称、住所及び安定供給確保支援業務を行う営業所又は事務所の所在地並びに指定に係る特定重要物資を公示するものとする。</Sentence>
    </ParagraphSentence>
  </Paragraph>
  <Paragraph Num="2">
    <ParagraphNum>２</ParagraphNum>
    <ParagraphSentence>
      <Sentence Num="1" WritingMode="vertical">安定供給確保支援法人は、その名称、住所又は安定供給確保支援業務を行う営業所若しくは事務所の所在地を変更するときは、あらかじめ、その旨を主務大臣に届け出なければならない。</Sentence>
    </ParagraphSentence>
  </Paragraph>
</Article>
</Section>
"#;

#[tokio::test]
async fn get_all_info_2() {
  let law_text_lst = vec![
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("主務大臣は、安定供給確保基本指針及び安定供給確保取組方針に基づき、主務省令で定めるところにより、一般社団法人、一般財団法人その他主務省令で定める法人であって、第三項に規定する業務（以下この章及び第九十六条第三号において「安定供給確保支援業務」という。）に関し次の各号のいずれにも適合すると認められるものを、その申請により、特定重要物資ごとに安定供給確保支援法人として指定することができる。".to_string())
    },
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("1".to_string()),
        item: Some("1".to_string()),
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("安定供給確保支援業務を適正かつ確実に実施することができる経理的基礎及び技術的能力を有するものであること。".to_string())
    },
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("1".to_string()),
        item: Some("2".to_string()),
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("安定供給確保支援業務の実施体制が安定供給確保基本指針に照らし適切であること。".to_string())
    },
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("次の各号のいずれかに該当する者は、前項の規定による指定（以下この節において「指定」という。）を受けることができない。".to_string())
    },
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("2".to_string()),
        item: Some("1".to_string()),
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("この法律の規定に違反し、刑に処せられ、その執行を終わり、又は執行を受けることがなくなった日から起算して二年を経過しない者".to_string())
    },
    LawText {
      article_info: Article {
        article: "31".to_string(),
        paragraph: Some("2".to_string()),
        item: Some("2".to_string()),
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("第四十一条第一項又は第二項の規定により指定を取り消され、その取消しの日から起算して二年を経過しない者".to_string())
    },
    LawText {
      article_info: Article {
        article: "32".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("主務大臣は、指定をしたときは、当該指定に係る安定供給確保支援法人の名称、住所及び安定供給確保支援業務を行う営業所又は事務所の所在地並びに指定に係る特定重要物資を公示するものとする。".to_string())
    },
    LawText {
      article_info: Article {
        article: "32".to_string(),
        paragraph: Some("2".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("安定供給確保支援法人は、その名称、住所又は安定供給確保支援業務を行う営業所若しくは事務所の所在地を変更するときは、あらかじめ、その旨を主務大臣に届け出なければならない。".to_string())
    },
  ];
  let gen_law_text_lst = xml_to_law_text(LAW_XML_2.as_bytes()).await.unwrap();
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

  let target = Article {
    article: "30".to_string(),
    paragraph: Some("1".to_string()),
    item: None,
    sub_item: None,
    suppl_provision_title: None,
  };

  let law_text_lst = vec![
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
      contents : LawContents::Text("第三条から第二十六条まで及び第二十七条から前条までの規定は、法第十八条第四項の規定により託送供給等約款で設定した料金を変更しようとする一般送配電事業者が、変更しようとする託送供給等約款で設定する料金を算定する場合に準用する。この場合において、次の表の上欄に掲げる規定中同表の中欄に掲げる字句は、それぞれ同表の下欄に掲げる字句に読み替えるものとする。".to_string())
    },
    LawText {
      article_info: Article {
        article: "30".to_string(),
        paragraph: Some("1".to_string()),
        item: None,
        sub_item: None,
        suppl_provision_title: None,
      },
    contents: LawContents::Table(vec![
      LawTable {
        row: vec![
          LawTableColumn {
            rowspan: 1,
            colspan: 1,
            contents: LawTableContents::Text("第三条".to_string()),
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
  let gen_law_text_lst = search_law_text(str.as_bytes(), &target).await.unwrap();
  assert_eq!(law_text_lst, gen_law_text_lst)
}
