use std::fmt;
enum Gender {
    Male,
    Female,
    NonBinary,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::NonBinary => "Non-Binary",
        };
        write!(f, "{text}")
    }
}

struct Account {
    pub name: String,
    pub number: u32,
    pub address: String,
    pub gender: Gender,
    pub balance: u32,
}

impl Account {
    pub fn new( name: String, number: u32,
                address: String, gender: Gender,
                balance: u32 ) -> Result<Self, &'static str> {

        if name.len() < 3 {
            return Err("Name too short");
        }
        if number == 0 {
            return Err("Invalid number");
        }
        if address.len() < 5 {
            return Err("Enter a valid address");
        }

        Ok(Self{
            name, number,
            address, gender, balance
        })
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), &'static str> {
        if name.len() < 3 {
            return Err("Enter a valid name!");
        }
        self.name = name.to_string();
        Ok(())
    }

    pub fn set_address(&mut self, addr: &str) -> Result<(), &'static str>{
        if addr.chars().count() < 5 {
            return Err("Enter a valid name!");
        }
        self.address = addr.to_string();
        Ok(())
    }

    pub fn set_number(&mut self, num: u32) -> Result<(), &'static str> {
        if num == 0 { 
            return Err("Enter a valid phone number!"); 
        }
        self.number = num;
        Ok(())
    }

    pub fn set_gender(&mut self, gender: Gender) {
        self.gender = gender;
    }

    pub fn deposit(&mut self, amount: u32) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("Deposit a valid amount!");
        }
        self.balance += amount;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u32) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("Enter a valid amount!");
        }

        if amount > self.balance {
            return Err("You do not have enough amount!");
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn display(&self) {
        println!("Name: {}\nAddress: {}\nNumber: {}\nGender: {}\nBalance: {}", 
        self.name, self.address, self.number, self.gender, self.balance);
    }
}

fn main() -> Result<(), &'static str> {
    let mut acc1  = Account::new("Bob Ross".to_string(), 14111341, "Florida".to_string(), Gender::NonBinary, 0)?;

    acc1.deposit(92000)?;
    acc1.deposit(1100)?;
    acc1.withdraw(90000)?;
    acc1.set_gender(Gender::Female);
    acc1.set_address("Texas")?;
    acc1.set_name("Blue")?;

    acc1.display();
    Ok(())
}