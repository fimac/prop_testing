use postgres::{Client, NoTls};
use quickcheck::Arbitrary;
use quickcheck::Gen;
use quickcheck_macros::quickcheck;

// #[derive(Debug, Clone)]
// struct InsertInt {
//     value: i32,
// }
#[derive(Debug, Clone)]
struct InsertInt {
    number: i32,
}
// #[derive(Debug, Clone)]
// struct InsertString {
//     string: String,
// }

impl Arbitrary for InsertInt {
    fn arbitrary(g: &mut Gen) -> Self {
        InsertInt {
            number: i32::arbitrary(g),
        }
    }
}
// impl Arbitrary for InsertString {
//     fn arbitrary(g: &mut Gen) -> Self {
//         InsertString {
//             string: String::arbitrary(g),
//         }
//     }
// }
fn insert_and_verify(data_1: InsertInt) -> Result<bool, postgres::Error> {
    println!("hitting here ");
    let mut client = Client::connect("host=localhost user=postgres dbname=sql_testing", NoTls)?;
    println!("dataaaa {:?}", data_1);
    client.execute("CREATE TABLE IF NOT EXISTS prop_tesing (number INT)", &[])?;
    println!("table created");
    client.execute(
        "INSERT INTO prop_tesing (number) VALUES ($1)",
        &[&data_1.number],
    )?;
    for row in client.query(
        "SELECT number FROM prop_tesing WHERE number = $1",
        &[&data_1.number],
    )? {
        let retrieved_number: i32 = row.get(0);
        println!("retrieved number: {:?}", retrieved_number);
        println!("rando number {:?}", data_1.number);
        assert_eq!(retrieved_number, data_1.number);
    }
    Ok(true)
}

// fn insert_and_verify(data_1: InsertInt, data_2: InsertString) -> Result<bool, postgres::Error> {
//     println!("hitting here ");
//     let mut client = Client::connect("host=localhost user=postgres dbname=sql_testing", NoTls)?;
//     println!("dataaaa {:?}", data_1);
//     client.execute(
//         "CREATE TABLE IF NOT EXISTS prop_testing (number INT, string TEXT)",
//         &[],
//     )?;
//     println!("table created");
//     client.execute(
//         "INSERT INTO prop_tesing (number, string) VALUES ($1, $2)",
//         &[&data_1.number, &data_2.string],
//     )?;
//     for row in client.query(
//         "SELECT number FROM prop_tesing WHERE number = $1",
//         &[&data_1.number],
//     )? {
//         let retrieved_number: i32 = row.get(0);
//         println!("retrieved number: {:?}", retrieved_number);
//         println!("rando number {:?}", data_1.number);
//         assert_eq!(retrieved_number, data_1.number);
//     }
//     for row in client.query(
//         "SELECT string FROM prop_tesing WHERE stromg = $1",
//         &[&data_2.string],
//     )? {
//         let retrieved_string: String = row.get(0);
//         assert_eq!(retrieved_string, data_2.string);
//     }
//     Ok(true)
// }

// #[quickcheck]
// fn test_insertion(data_1: InsertInt, data_2: InsertString) -> bool {
//     match insert_and_verify(data_1, data_2) {
//         Ok(result) => result,
//         Err(_) => false,
//     }
// }

#[quickcheck]
fn test_insertion(data_1: InsertInt) -> bool {
    match insert_and_verify(data_1) {
        Ok(result) => result,
        Err(_) => false,
    }
}

fn main() {
    println!("testing");
}
