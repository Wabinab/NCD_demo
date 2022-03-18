use crate::*;


pub fn article_new_default(
  owner_id: AccountId,
  article_number: u64,
  contract_id: AccountId,
) -> Article {
  Article {
    royalty: default_royalty(owner_id.clone(), contract_id, article_number.clone()),
    // royalty: UnorderedMap::new(
    //   StorageKey::RoyaltyKey.try_to_vec().unwrap()
    // ),
    article_id: format!("{}{}", owner_id.clone(), article_number),
    owner_id
  }
}


// pub trait NewArticle {
//   fn new_default(
//     &mut self,
//     owner_id: AccountId, 
//     article_number: u64,
//   ) -> Self;
// }


// impl NewArticle for Article {
//   fn new_default(
//     &mut self,
//     owner_id: AccountId, 
//     article_number: u64,
//   ) -> Self {
//     Self {
//       // as we need owner_id.clone(), we move to first place. 
//       royalty: default_royalty(owner_id.clone()),
//       article_id: format!("{}{}", owner_id.clone(), article_number), 
//       owner_id,
//     }
//   }
// }

// =========================================== //


#[near_bindgen]
impl Contract {
    pub fn add_new_article_default(
      &mut self,
      owner_id: AccountId,
      article_number: u64
    ) {
      let article = article_new_default(
        owner_id, 
        article_number, 
        self.owner_id.clone()
      );
      self.article_by_id.insert(&article.article_id, &article);
    }
}

// pub trait ArticleAdder {
//     fn add_new_article_default(
//       &mut self,
//       owner_id: AccountId,
//       article_number: u64,
//     );
// }


// impl ArticleAdder for Contract {
//     fn add_new_article_default(
//       &mut self,
//       owner_id: AccountId,
//       article_number: u64,
//     ) {
//       // Create the new article
//       eprintln!("start");
//       // let mut article = Article::default();
//       // eprintln!("continue");
//       // let article = article.new_default(owner_id, 54);
//       eprintln!("owner_id: {}", owner_id);
//       let article = article_new_default(owner_id, article_number);

//       // Add to LookupMap
//       eprintln!("Pass");
//       self.article_by_id.insert(&article.article_id, &article);
//       eprintln!("Pass");
//     }
// }