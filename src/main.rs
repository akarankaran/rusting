use std::io;

struct Account {
    balance: f64,
}

impl Account {
    fn new(initial_balance: f64) -> Self {
        Account { balance: initial_balance }
    }
    
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Deposit amount must be positive.".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Withdrawal amount must be positive.".to_string());
        }
        if amount > self.balance {
            return Err("Insufficient funds.".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account = Account::new(1000.0);
    loop {
        println!("Current balance: {}", account.get_balance());
        println!("Choose an action: 1) Deposit 2) Withdraw 3) Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);
        
        match choice {
            1 => {
                let mut amount = String::new();
                println!("Enter deposit amount:");
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                match account.deposit(amount) {
                    Ok(_) => println!("Deposited: {}", amount),
                    Err(e) => println!("Error: {}", e),
                }
            }
            2 => {
                let mut amount = String::new();
                println!("Enter withdrawal amount:");
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().unwrap_or(0.0);
                match account.withdraw(amount) {
                    Ok(_) => println!("Withdrawn: {}", amount),
                    Err(e) => println!("Error: {}", e),
                }
            }
            3 => {
                println!("Exiting. Final balance: {}", account.get_balance());
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}