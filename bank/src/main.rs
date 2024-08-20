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
    accounts: Vec<String>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_acc(account: Account) {
    println!("{:#?}", account);
}

fn print_acc_ref(account: &Account) {
    println!("{:#?}", account);
}


fn print_list_acc(account: Vec<Account>) {
    println!("{:#?}", account);
}


fn change_account(account: &mut Account) {
    account.balance = 20;
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let other_bank = bank;
    
    let account = Account::new(1, String::from("H001")); 
    // assign reference (immutable / read-only.)
    // let account_ref_1 = &account;
    // let account_ref_2 = &account;

    // mutable reference.
    // let mut mut_account = Account::new(1, String::from("H001")); 
    // change_account(&mut mut_account);
    // print_acc(account);

    //let lis_account = vec![account];

    // println!("{:#?}", other_bank);
    // print_list_acc(lis_account);
    // print_acc_ref(account_ref_1);
    // print_acc_ref(account_ref_2);

    // breaks ownership
    let num = 5;
    let other_num = num;

    println!("{} {}", num, other_num);
    
}
