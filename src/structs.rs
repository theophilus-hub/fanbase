use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::{near_bindgen, env};
use near_sdk::AccountId;
use near_sdk::serde::Serialize;


#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct UserDetails {
    pub name: String,
    pub username: String,
    pub bio: String,
    pub image: String,
    pub vip_price: u128
}


#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Transactions {
    pub purchases : Vec<String>
}


#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Content{
    pub content : Vec<String>
}


#[derive( BorshDeserialize, BorshSerialize)]
pub struct Memberlist{
   pub regular: LookupMap<AccountId,AccountId>,
   pub vip: LookupMap<AccountId,AccountId>,
}



#[derive(BorshDeserialize, BorshSerialize)]

pub struct User {
    pub details : UserDetails,
    pub tx: Transactions,
    pub content : Content,
    pub members: Memberlist
}





impl User {
    // #[near_sdk::result_serializer(borsh)]
    pub fn account(&mut self, name: String, username: String, bio: String, image: String ) -> User {
       
        //PACKAGING THE DATA FOR THE CREATING OF ACCOUNTS
        let user_details = UserDetails{
        name: name,
        username: username,
        bio: bio,
        image: image,
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

       //CREATING THE USER ACCOUNT 
       let user = User{
        details : user_details,
        tx : transactions,
        content : content,
        members : members
       };
       return user;


    }

    
}
