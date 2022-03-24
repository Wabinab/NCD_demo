use crate::*;


pub trait NewArticle {
  fn new_default(
    owner_id: AccountId, 
    article_number: u64,
  ) -> Self;
}


impl NewArticle for Article {
    fn new_default(
      owner_id: AccountId, 
      article_number: u64,
    ) -> Self {
      Self {
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
      article_number: u64
    ) {
      let article = Article::new_default(
        env::signer_account_id(), 
        article_number, 
      );
      self.article_by_id.insert(&article.article_id, &article);
    }
}