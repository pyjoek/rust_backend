struct Bank {
    owner: String,
    balance: f64
}

impl Bank {
    fn checkBalance(&self) {
        println!("Current Balance is {}",self.balance)
    }
}

fn main() {
    let account = Bank {
        owner: "Joel".to_string(),
        balance: 2400.00
    };

    let balan = &account.checkBalance();

    println!("Owner {}, Current balance is {:?}", account.owner, balan);
}