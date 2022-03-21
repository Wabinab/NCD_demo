use crate::*;


// pub fn article_new_default(
//   owner_id: AccountId,
//   article_number: u64,
//   contract_id: AccountId,
// ) -> Article {
//   Article {
//     royalty: default_royalty(owner_id.clone(), contract_id, article_number.clone()),
//     article_id: format!("{}{}", owner_id.clone(), article_number),
//     owner_id
//   }
// }


pub trait NewArticle {
  fn new_default(
    owner_id: AccountId, 
    article_number: u64,
    // contract_id: AccountId,
  ) -> Self;
}


impl NewArticle for Article {
    fn new_default(
      owner_id: AccountId, 
      article_number: u64,
      // contract_id: AccountId
    ) -> Self {
      Self {
        // royalty: default_royalty(owner_id.clone(), contract_id, article_number.clone()),
        royalty: HashMap::new(),
        article_id: format!("{}{}", owner_id.clone(), article_number), 
        owner_id,
      }
    }
}

// =========================================== //


#[near_bindgen]
impl Contract {
    pub fn add_new_article_default(
      &mut self,
      // owner_id: AccountId,
      article_number: u64
    ) {
      let owner_id = env::signer_account_id();

      let article = Article::new_default(
        owner_id, 
        article_number, 
        // self.owner_id.clone()
      );
      self.article_by_id.insert(&article.article_id, &article);
    }
}