use postgres::{Client, NoTls, Error};

fn main() {
    let mut client = Client::connect("postgresql://postgres:jam@localhost:5432/prodBackupsDBbbb", NoTls);
    println!("Connected");
}
