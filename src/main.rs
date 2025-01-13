use std::io;

struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new() -> BankAccount {
        BankAccount { balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited: {}", amount);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!("Withdrew: {}", amount);
            } else {
                println!("Insufficient funds for withdrawal.");
            }
        } else {
            println!("Withdrawal amount must be positive.");
        }
    }

    fn check_balance(&self) {
        println!("Current balance: {}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new();

    loop {
        println!("Choose an option: 1) Deposit 2) Withdraw 3) Check Balance 4) Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: i32 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                let mut amount = String::new();
                println!("Enter deposit amount: ");
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().unwrap();
                account.deposit(amount);
            },
            2 => {
                let mut amount = String::new();
                println!("Enter withdrawal amount: ");
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().unwrap();
                account.withdraw(amount);
            },
            3 => {
                account.check_balance();
            },
            4 => {
                break;
            },
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}