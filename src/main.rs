#![allow(warnings)]
use std::io;

struct Accounts<'a> {
    account_number: String,
    account_balance: f64,
    account_belongs_to_user: &'a Users<'a>, 
    account_belongs_to_bank: &'a Banks<'a>, 
}

impl<'a> Accounts<'a> {
    pub fn new (account_number: String, account_balance: f64, account_belongs_to_user: &'a Users<'a>, account_belongs_to_bank: &'a Banks<'a>) -> Accounts<'a> {
        Accounts{
            account_number,
            account_balance,
            account_belongs_to_user,
            account_belongs_to_bank,
        }
    }
    pub fn show_balance(&self) {
        println!("Account balance : {}", self.account_balance); 
    }
    pub fn deposit(&mut self, amount: f64) {
        self.account_balance += amount;
    }
    pub fn withdraw(&mut self, amount: f64) {
        if self.account_balance >= amount {
            self.account_balance -= amount;
        } else {
            println!("Insufficient balance");
        }
    }

}

struct Banks<'a> {
    bank_name: String,
    accounts: Vec<Accounts<'a>>
}

impl<'a> Banks<'a> {
    pub fn new (bank_name: String, accounts: Vec<Accounts<'a>>) -> Self {
        Self{
            bank_name,
            accounts,
        }
    }
}

struct Users<'a> {
    u_name: String,
    accounts: Vec<Accounts<'a>>,  
}

impl<'a> Users<'a> {
    pub fn new (u_name: String, accounts: Vec<Accounts<'a>>) -> Self {
        Self{
            u_name,
            accounts,
        }
    }
}

pub fn show_accounts_all_information(acc: &Accounts)
{
    println!("Account number: {} \n Account balance {} \n Account belongs to bank {} \n Account belong to user {}",
    acc.account_number, acc.account_balance, acc.account_belongs_to_bank.bank_name, acc.account_belongs_to_user.u_name);
}

pub fn deposit_interest_for_a_month(acc: &mut Accounts, mut amount :f64) {
    println!("\t\t\t Yüzde 12 faiz işlemi");
    if(amount>acc.account_balance)
        {println!("Amount is bigger than balance. Correct it");}
    else
    {
        acc.account_balance-=amount;
        amount=amount*112.0/100.0;
        acc.account_balance+=amount;
    }
}

pub fn demonstrate_balance(account: &Accounts) {
    println!("Account balance: {}", account.account_balance);
}

pub fn transfer_money<'a>(from: &mut Accounts<'a>, to: &mut Accounts<'a>, amount: f64) {
    if from.account_balance >= amount {
        from.account_balance -= amount;
        to.account_balance += amount;
    } else {
        println!("Insufficient balance");
    }
}

pub fn request_money<'a>(from: &mut Accounts<'a>, to: &mut Accounts<'a>, amount: f64){
    if to.account_balance >= amount {
        to.account_balance -= amount;
        from.account_balance += amount;
    } else {
        println!(" {} doesnt have enough balance", to.account_belongs_to_user.u_name);
    }
}

fn main() {  
    let bank1 = Banks::new("Ziraat Bankasi".to_string(), vec![]);
    let bank2 = Banks::new("Garanti Bankasi".to_string(), vec![]);

    let user1 = Users::new("Dogus".to_string(), vec![]);
    let user2 = Users::new("Ali".to_string(), vec![]);
    let user3 = Users:: new("Veli".to_string(), vec![]);


    let mut account1= Accounts::
    new("123456789".to_string(), 1000.0, &user1, &bank1);
    let mut account2= Accounts::
    new("987654321".to_string(), 200.0, &user1, &bank1);
    let mut account3= Accounts::
    new("7777777".to_string(), 500.0, &user2, &bank2);
  
    println!("Kullanıcıların isimlerini ve hesap numaralarını hesapları kullanarak yazdırma");
    println!("User name: {} \tAccount number: {}", account1.account_belongs_to_user.u_name, account1.account_number);
    println!("User name: {} \tAccount number: {}", account2.account_belongs_to_user.u_name, account2.account_number);
    println!("User name: {}  \tAccount number: {}\n\n", account3.account_belongs_to_user.u_name, account3.account_number);

    println!("Hesapların ait olduğu bankaları ve hesap numaralarını hesapları kullanarak yazdırma");
    println!("Bank name: {} \tAccount number: {}", account1.account_belongs_to_bank.bank_name, account1.account_number);
    println!("Bank name: {} \tAccount number: {}", account2.account_belongs_to_bank.bank_name, account2.account_number);
    println!("Bank name: {} \tAccount number: {}", account3.account_belongs_to_bank.bank_name, account3.account_number);

    println!("\n\nHesap 1 den hesap 2 ye 500 birim para transferi\n Transfer öncesi");
    demonstrate_balance(&account1);
    demonstrate_balance(&account2);
    transfer_money(&mut account1, &mut account2, 500.0);
    println!("Transfer sonrası");
    demonstrate_balance(&account1);
    demonstrate_balance(&account2);

    println!("\n\nRequest money örneği 200 birim");
    println!("Request öncesi");
    demonstrate_balance(&account1);
    demonstrate_balance(&account2);
    request_money(&mut account1, &mut account2, 200.0);
    println!("Request sonrası");
    demonstrate_balance(&account1);
    demonstrate_balance(&account2);
  
    println!("\n\n");
    show_accounts_all_information(&account1);
    
    deposit_interest_for_a_month(&mut account1,300.0);
    show_accounts_all_information(&account1);
}
