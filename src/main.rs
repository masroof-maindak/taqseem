use std::collections::LinkedList;
use std::error::Error;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
struct Person {
    name: String,
    paid: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut people = LinkedList::new();

    let mut rdr = csv::Reader::from_reader(std::io::stdin());

    for result in rdr.deserialize() {
        let p: Person = result?;
        people.push_back(p);
    }

    let total_bill: usize = people.iter().map(|p| p.paid).sum();
    let bill_per_head = total_bill / people.len();

    println!("Total bill: {total_bill}");
    println!("Bill per head: {bill_per_head}");
    println!();

    let deservees: LinkedList<Person> = people.extract_if(|p| p.paid > bill_per_head).collect();
    let mut underpaid: LinkedList<Person> = people.extract_if(|p| p.paid < bill_per_head).collect();

    // assign an underpaid to a deservee
    for deservee in deservees.iter() {
        let mut pending_amount_to_recv = deservee.paid - bill_per_head;

        println!(
            "`{}` paid extra @ {}; they deserve {}",
            deservee.name, deservee.paid, pending_amount_to_recv
        );

        // see how much each underpaid should give this guy, until their excess payment is covered
        for poor in &mut underpaid {
            let to_pay_deservee = std::cmp::min(pending_amount_to_recv, bill_per_head - poor.paid);

            println!(
                "\t`{}` (paid {} so far) has to pay `{}`, {to_pay_deservee}",
                poor.name, poor.paid, deservee.name,
            );

            pending_amount_to_recv -= to_pay_deservee;

            poor.paid += to_pay_deservee;

            if pending_amount_to_recv == 0 {
                break;
            }
        }

        let _poors_that_paid = underpaid
            .extract_if(|p| p.paid == bill_per_head)
            .collect::<LinkedList<Person>>();
    }

    Ok(())
}
