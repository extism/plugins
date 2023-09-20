use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Money {
    pub amount_in_cents: u64,
    pub currency: String,
}

#[derive(Deserialize)]
struct Customer {
    pub customer_id: String,
    pub full_name: String,
    pub total_spend: Money,
    pub credit: Money,
}

#[derive(Deserialize)]
struct ChargeSuceededEvent {
    pub event_type: String,
    pub customer: Customer,
}

#[derive(Serialize)]
struct Email<'a> {
    pub subject: &'a str,
    pub body: &'a str,
}

#[host_fn]
extern "ExtismHost" {
    fn add_credit(customer_id: String, amount: Json<Money>) -> Json<Customer>;
    fn send_email(customer_id: String, email: Json<Email>);
}

const ONE_HUNDRED_DOLLARS: u64 = 10_000; // in cents

#[plugin_fn]
pub fn on_charge_succeeded(Json(event): Json<ChargeSuceededEvent>) -> FnResult<()> {
    let customer = event.customer;
    let total_spend = customer.total_spend;

    if total_spend.currency.to_uppercase() == "USD"
        && total_spend.amount_in_cents > ONE_HUNDRED_DOLLARS
        && customer.credit.amount_in_cents == 0
    {
        let credit = Money {
            amount_in_cents: 1_000, // $10 in cents
            currency: "USD".into(),
        };
        let _customer = unsafe { add_credit(customer.customer_id.clone(), Json(credit))? };

        let email = Email {
            subject: &format!("A gift for you {}", customer.full_name),
            body: "You have received $10 in store credit!",
        };
        unsafe { send_email(customer.customer_id, Json(email))? };
    }

    Ok(())
}
