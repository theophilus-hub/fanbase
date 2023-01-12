

//NECESSARY IMPORTATIONS//
//......................//
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, LookupMap};
use near_sdk::{near_bindgen, AccountId, Promise, PromiseResult};
use near_sdk::json_types::{U128};
use near_sdk::PanicOnDefault;
use near_sdk::env::{predecessor_account_id, signer_account_id, account_balance, self};
mod structs;
use structs::{UserDetails,Transactions, Content, Memberlist, User};



//MAIN CONTRACT CLASS//
//...................//
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    acc_collection : LookupMap<AccountId, User>,
    links : LookupMap<AccountId, AccountId>
}



//CONTRACT DEFAULT STATE//
//......................//
impl Default for Contract{
    fn default() -> Self {
        Self { 
            acc_collection: LookupMap::new(b"".to_vec()),
            links: LookupMap::new(b"".to_vec())
         }
    }
}



//MAIN CONTRACT CLASS IMPLIMENTATION//
//..................................//
#[near_bindgen]
impl Contract {

    // INIT FUNCTION//
    //..............//
    #[init]
    #[private] 
    pub fn new() -> Self {
      assert!(!env::state_exists(), "Already initialized");
      Self {
        acc_collection: LookupMap::new(b"map-uid-1".to_vec()),
        links: LookupMap::new(b"map-uid-1".to_vec())
      }
    }






//.................//// PUBLIC FUNCTIONS////.................//
    


//...............///CREATE AND GET USER DETAILS///..................//
//...............///CREATE AND GET USER DETAILS///..................//



    //CREATE USER ACCOUNT //
    //....................//
    pub fn create(&mut self,name: String, username: String, bio: String, image: String){

        let user_details = UserDetails{
            name: name.clone(),
            username: username.clone(),
            bio: bio.clone(),
            image: image.clone(),
            vip_price: 0
           };
    
           let transactions = Transactions {
            purchases: vec![]
           };
    
           let content = Content {
            content: vec![]
           };
    
           let members = Memberlist {
            regular: LookupMap::new(b"m".to_vec()),
            vip: LookupMap::new(b"m".to_vec())
           };
        let user = User::account(&mut User { details: user_details, tx: transactions, content: content, members: members }, name, username, bio, image);
       
        let Contract_caller = predecessor_account_id();
        let contract_signer = signer_account_id();

        if Contract_caller == contract_signer{
          self.acc_collection.insert(&predecessor_account_id(), &user);
        }
        let id = 1;

    }


  
    //GET USER ACCOUNT//
    //................//
    #[result_serializer(borsh)]
    pub fn get_user_account(&self) -> User{

      let query_account = predecessor_account_id();
      self.acc_collection.get(&query_account).unwrap()

    }



    //GET USER ACCOUNT TRANSACTIONS//
    //.............................//
    #[result_serializer(borsh)]
    pub fn get_user_details(&self) -> UserDetails{
      
      let query_account = predecessor_account_id();
      self.acc_collection.get(&query_account).unwrap().details

    }
    


    //GET USER ACCOUNT TRANSACTIONS//
    //.............................//
    #[result_serializer(borsh)]
    pub fn get_user_tx(&self) -> Transactions{

      let query_account = predecessor_account_id();
      self.acc_collection.get(&query_account).unwrap().tx

    }





    //...............///FOLLOWERS FUNCTIONS///..................//
    //...............///FOLLOWERS FUNCTIONS///..................//



    //GET USER ACCOUNT FOLLOWER STATUS//
    //............................//
    #[result_serializer(borsh)]
    pub fn get_follow_status(&self, creator_accountid : AccountId) -> bool{

      let query_account = predecessor_account_id();

      self
      .acc_collection
      .get(&creator_accountid)
      .unwrap()
      .members
      .regular
      .get(&query_account).is_some()
    }



    //GET NUMBER OF FOLLOWERS//
    //.......................//
    #[result_serializer(borsh)]
    pub fn get_user_content(&self, query_account: AccountId) -> usize{

     let map = self
      .acc_collection
      .get(&query_account)
      .unwrap()
      .members
      .regular;
      
      map.try_to_vec().iter().count()

    }



    //FOLLOW FUNCTION  //
    //.................//
    pub fn follow(&self, creator_accountid : AccountId){
     if creator_accountid != predecessor_account_id() {
      self
      .acc_collection.get(&creator_accountid)
      .unwrap()
      .members
      .regular
      .insert(&predecessor_account_id(), &creator_accountid);

      // self
      // .links
      // .insert(&predecessor_account_id(), &creator_accountid);
     }
    }

    






//...............///SUBSCRIBER FUNCTIONS///..................//
//...............///SUBSCRIBER FUNCTIONS///..................//



    //SET SUBSCRIPTION PRICE FUNCTION  //
    //.................................//
    pub fn set_vip_price(&self, price: u128){

      self
      .acc_collection
      .get(&predecessor_account_id())
      .unwrap()
      .details
      .vip_price = price;
    }



    //GET SUBSCRIPTION PRICE FUNCTION  //
    //.................................//
    #[result_serializer(borsh)]
    pub fn get_vip_price(&self, query_account: AccountId) -> u128{

      self
      .acc_collection
      .get(&query_account)
      .unwrap()
      .details
      .vip_price
    }




    // SUBSCRIPTION FUNCTION  //
    //........................//

    fn check_promise_result(result: PromiseResult) {
      match result {
        PromiseResult::NotReady => unreachable!(),

          PromiseResult::Successful(val)=> {
              // The promise was successful
              // Do something here to handle the success case
          }
          PromiseResult::Failed => {
              // The promise failed
              // Do something here to handle the error case
          }
      }
  }

    #[result_serializer(borsh)]
    #[payable]
    pub fn subscribe(&mut self, query_account: AccountId) {

      let vip_price = self.get_vip_price(query_account.clone());
      let balance = account_balance();

      assert!(balance >= vip_price, "Insufficient balance");
      let promise_object = Promise::new(query_account).transfer(vip_price);
    
      
    }

}
