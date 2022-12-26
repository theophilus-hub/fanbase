
//NECESSARY IMPORTATIONS//
//......................//
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, LookupMap};
use near_sdk::{near_bindgen, AccountId};
use near_sdk::json_types::{U128};
use near_sdk::PanicOnDefault;
use near_sdk::env;
use near_sdk::env::{predecessor_account_id, signer_account_id};
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

    //CREATE USER ACCOUNT //
    //....................//
    pub fn create(&mut self,name: String, username: String, bio: String, image: String){

        let user_details = UserDetails{
            name: name.clone(),
            username: username.clone(),
            bio: bio.clone(),
            image: image.clone(),
           };
    
           let transactions = Transactions {
            purchases: vec![]
           };
    
           let content = Content {
            content: vec![]
           };
    
           let members = Memberlist {
            regular: vec![],
            vip: vec![]
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


    //GET USER ACCOUNT SUBSCRIBERS//
    //............................//
    #[result_serializer(borsh)]
    pub fn get_user_members(&self) -> Memberlist{

      let query_account = predecessor_account_id();
      self.acc_collection.get(&query_account).unwrap().members

    }


    //GET USER ACCOUNT SUBSCRIBERS//
    //............................//
    #[result_serializer(borsh)]
    pub fn get_user_content(&self) -> Content{

      let query_account = predecessor_account_id();
      self.acc_collection.get(&query_account).unwrap().content

    }




//........................///MODIFY USER ACCOUNT DETAILS///.........................//

    //SUBSCRIBE FUNCTION  //
    //...........//
    pub fn subscribe(&self, creator_accountid : AccountId){
     if creator_accountid != predecessor_account_id() {
      self
      .acc_collection.get(&creator_accountid)
      .unwrap()
      .members
      .regular
      .push(predecessor_account_id());

      // self
      // .links
      // .insert(&predecessor_account_id(), &creator_accountid);
     }
    }



}


