#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_num_accounts(bank: &Bank) {
    println!("{:?}", bank.accounts.len());
}

fn main() {
    let mut bank = Bank::new();
    let account_1 = Account::new(1, String::from("me"));
    let account_2 = Account::new(2, String::from("me"));
    print_num_accounts(&bank);
    bank.accounts.push(account_1);
    bank.accounts.push(account_2);
    print_num_accounts(&bank);
}
