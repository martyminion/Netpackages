use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk:: collections::{UnorderedMap,Vector};
use near_sdk:: {env,near_bindgen,AccountId};
use near_sdk:: serde::{Deserialize,Serialize};


//This is a internet user subscription app
// The customer chooses the preferred package; 
//We are able to know when they paid and when the subscription will end.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//We describe the packages
enum NetOptionsPackages {
    None,
    Bronze,
    Silver,
    Gold,
}

impl Default for NetOptionsPackages {
    fn default() -> Self {
        NetOptionsPackages::None
    }
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
//We define the characteristics of each customer, required information
pub struct Customers {
    account: AccountId,
    name: String,
    phone_number: String,
    house_number: String,
    net_option: NetOptionsPackages,
    date_paid: u64,
    due_date: u64,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NetOptions {
    packages: UnorderedMap<NetOptionsPackages,i64>,
    customers: Vector<Customers>,
}

impl Default for NetOptions {
    fn default() -> Self {
        let mut package = UnorderedMap::new(b"p".to_vec());

        //We set the pricess for each package
        package.insert(&NetOptionsPackages::Gold, &3000);
        package.insert(&NetOptionsPackages::Silver, &2500);
        package.insert(&NetOptionsPackages::Bronze, &2000);
        
        NetOptions { 
            customers: Vector::new(b"r".to_vec()),
            packages: package,
    
        }
    }
}

#[near_bindgen]
impl NetOptions {
    //Function to add a new customer
    pub fn add_customer(
        &mut self,
        name: String,
        phone: String,
        house_number: String,
        package: String,
    ) {
        let _bronze = "bronze".to_string();
        let _silver = "silver".to_string();
        let _gold = "gold".to_string();
        
        let mut _option: Option<NetOptionsPackages>=None;

        if package.eq(&_bronze){
            _option=Some(NetOptionsPackages::Bronze)
        } else if package.eq(&_silver){
            _option=Some(NetOptionsPackages::Silver)
        } else if package.eq(&_gold){
            _option=Some(NetOptionsPackages::Gold)
        } else{
            _option=Some(NetOptionsPackages::None)
        }


        let cs = Customers {
            account: env::current_account_id(),
            name: name,
            phone_number: phone,
            house_number: house_number,
            date_paid: env::block_timestamp(),
            due_date: env::block_timestamp() + 60*60*24*30,
            net_option: _option.unwrap(),
            
        };
        self.customers.push(&cs);
    }
    //Function to retrieve all customer information
    pub fn all_customer(&self) -> Vec<Customers> {
        let mut tmp : Vec<Customers> = vec![];

        for item in self.customers.iter()  {
            tmp.push(item);
        }
        return tmp;
    }
    //Function to  retrieve an individual customer
    pub fn get_customer_by_phone(&self, phone: String) -> Option<Customers> {

        let mut a_customer :Option<Customers> = None;

        for elem in self.customers.iter() {
            if elem.phone_number == phone{
                a_customer = Some(elem);
            }
        }

        return a_customer;
    }
    //Function to update cutomer package, when customer is changing packages
    pub fn update_customer_package(
        &mut self,
        phone: String,
        package: String,
    ) {

        let _bronze = "bronze".to_string();
        let _silver = "silver".to_string();
        let _gold = "gold".to_string();

        let mut _option: Option<NetOptionsPackages>=None;

        if package.eq(&_bronze){
            _option=Some(NetOptionsPackages::Bronze)
        } else if package.eq(&_silver){
            _option=Some(NetOptionsPackages::Silver)
        } else if package.eq(&_gold){
            _option=Some(NetOptionsPackages::Gold)
        } else{
            _option=Some(NetOptionsPackages::None)
        }

        let mut customer_index: Option<u64> = None;
        for (index, customer) in self.customers.iter().enumerate() {
            if customer.phone_number == phone {
               
                customer_index = Some(index as u64);
                
                break;
            }
        }
        match customer_index {
            Some(c_index) => {
    
                let mut cust = self.customers.get(c_index).unwrap();
    
                let x = _option.unwrap();
    
                cust.net_option = x;
    
                self.customers.replace(c_index, &cust);
            }
    
            None => {
                env::panic_str("Customer does not exist")
            }
        }
    }
    

}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{ VMContextBuilder};
    use near_sdk::{ AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
   #[test]
   fn add_customer(){
    let user = AccountId::new_unchecked("martyminion.testnet".to_string());
    let mut context = get_context(user.clone());
    context.block_timestamp(9999);


    let mut  app = NetOptions::default();
    app.add_customer(
        "Andrew".to_string(),
        "111".to_string(), 
        "001".to_string(), 
        "gold".to_owned()
    );

    assert_eq!(app.customers.len(), 1)
   }


   #[test]
   fn update_customer(){

    let user = AccountId::new_unchecked("martyminion.testnet".to_string());
    let mut context = get_context(user.clone());
    context.block_timestamp(9999);

    let mut  app = NetOptions::default();
    
    app.add_customer(
        "Joe".to_string(), 
        "111".to_string(), 
        "001".to_string(),
        "bronze".to_owned());
//update Customer
    app.update_customer_package("111".to_string(),"silver".to_string());

// get customer and check package
    
let red = app.get_customer_by_phone("111".to_owned());
match red {
    Some(yell)=>{
        assert_eq!(yell.net_option,NetOptionsPackages::Silver)
    } 
    None=>{
        panic!("Customer Not Found")
    }
}    


   }


}
