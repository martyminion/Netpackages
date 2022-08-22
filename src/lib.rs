use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize,Debug};
use near_sdk::near_bindgen;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]

enum NetOptionsPackages { 
    None,
    Bronze,
    Silver,
    Gold,
}


impl Default for NetOptionsPackages{
    fn default() -> Self {
        NetOptionsPackages::None
    }
} 

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Customers {
    // SETUP CONTRACT STATE
    name: String,
    phone_number: String,
    house_number: String,
    net_option: NetOptionsPackages,
    date_paid: String,
    due_date: String,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NetOptions {
    customers: Vec<Customers>,
}


impl Default for NetOptions{
    fn default() -> Self {
        NetOptions {customers: vec![]}
    }
}

#[near_bindgen]
impl NetOptions {
    // ADD CONTRACT METHODS HERE
    fn add_customer(
        &mut self,
        name:String,
        phone: String,
        house_number: String,
        start: String,
        due: String,
        package: String,
    ) {
        let _bronze = String::from("small");
        let _silver = String::from("small");
        let _gold = String::from("small");
        let cs = customers {
            name: name,
            phone_number:phone,
            house_number:house_number,
            net_option: match package {
                _bronze => NetOptionsPackages::Bronze,
                _silver => NetOptionsPackages::Silver,
                _gold => NetOptionsPackages::Gold,
                _ => NetOptionsPackages::None,
            },
            date_paid: start,
            due_date: due,

        };
        self.customers.push(cs);
    }
    fn update_customer(
        &mut self,
        phone:String,
        package: String,
        date_paid: String,
        date_due: String,
    ){
        let _bronze = "bronze".to_string();
        let _silver = "silver".to_string();
        let _gold = "gold".to_string();

        for customer in self.customers.iter_mut() {
            if customer.phone_number == phone {
               
                customer.date_paid = date_paid;
                customer.due_date = date_due;
                let x = match &package {
                _bronze => NetOptionsPackages::Bronze,
                _silver => NetOptionsPackages::Silver,
                _gold => NetOptionsPackages::Gold,
                _=> NetOptionsPackages::None,

                };
            }

        }
    }
    fn get_customer(&self, phone: String) -> Option<&Customers>{
        let mut a_customer :Option<&Customers> = None;

        for elem in self.customers.iter() {
            if elem.phone_number == phone {
                a_customer = Some(elem);
            }
        }
        return a_customer;
    }
    fn all_customer(&self) -> Vec<&Customers> {
        let mut tmp : Vec<&Customers> = vec![];

        for item in self.customers.iter()  {
            tmp.push(item);
        }
        return tmp;
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
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn add_customer() {
        let mut app = NetOptions::default();
        app.add_customer("kenn".to_string(), "111".to_string(), "001".to_string(), "21/07/2022".to_string(), "21/08/2022".to_string(), "bronze".to_owned());

    assert_eq!(app.customers.len(), 1)
    }
    #[test]
   fn update_customer(){
    let mut  app = NetOptions::default();
    
    app.add_customer("kenn".to_string(), "111".to_string(), "001".to_string(), "21/07/2022".to_string(), "21/08/2022".to_string(), "bronze".to_owned());
    //update Customer
    app.update_customer("111".to_string(),"gold".to_string(),"23/06/2022".to_string(), "23/07/2022".to_string());

    let red = app.get_customer("111".to_owned());
        match red {
            Some(yell)=>{
            assert_eq!(yell.net_option,NetOptionsPackages::Medium)
            } 
            None=>{
            panic!("Customer Not Found")
    
                }    
        }
    }


}
