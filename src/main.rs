use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 25,
    };
    let serialized = user.try_to_vec().unwrap();
    println!("Serialized: {:?}", serialized);

    let deserialized: User = User::try_from_slice(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
