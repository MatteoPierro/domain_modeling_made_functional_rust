
// Composition of Types
enum AppleVariety {
    GoldenDelicious,
    GrannySmith,
    Fuji,
}

enum BananaVariety {
    Cavendish,
    GrosMichel,
    Manzano
}

enum CherryVariety {
    Montmorency,
    Bing
}

struct FruitSnack {
    apple: AppleVariety,
    banana: BananaVariety,
    cherry: CherryVariety
}

// Simple type
// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types

struct ProductCode(String);

// Working with Types

struct Person {
    first: String,
    last: String
}

enum OrderQuantity {
    UnitQuantity(usize),
    KilogramQuantity(f64)
}

fn print_quantity(order_quantity: OrderQuantity) {
    match order_quantity {
        OrderQuantity::UnitQuantity(u_qty) => {
            println!("{} units", u_qty)
        }
        OrderQuantity::KilogramQuantity(kg_qty) => {
            println!("{} kg", kg_qty)
        }
    }
}

// Building a Domain Model by Composing Types

struct CheckNumber(usize);
struct CardNumber(String);

enum CardType {
    Visa,
    Mastercard
}

struct CreditCardInfo {
    card_type: CardType,
    card_number: CardNumber
}

enum PaymentMethod {
    Cash,
    Check(CheckNumber),
    Card(CreditCardInfo)
}

struct PaymentAmount(f64);

enum Currency {
    EUR,
    USD
}

struct Payment {
    amount: PaymentAmount,
    currency: Currency,
    method: PaymentMethod
}

struct UnpaidInvoice;
struct PaidInvoice;

type ConvertPaymentCurrency = fn(Payment, Currency) -> Payment;

// Modeling Optional Values

struct PersonalName {
    first_name: String,
    middle_name: Option<String>,
    last_name: String
}

// Modeling Errors

enum PaymentError {
    CardTypeNotRecognized,
    PaymentRejected,
    PaymentProviderOffline
}

type PayInvoice = fn(UnpaidInvoice, Payment) -> Result<PaidInvoice, PaymentError>;

fn pay_invoice(unpaid_invoice: UnpaidInvoice, payment: Payment) -> Result<PaidInvoice, PaymentError> {
    todo!()
}

// Modeling No Value at All

struct Customer;
type SaveCustomer = fn(Customer);

type NextRandom = fn() -> usize;

fn main() {
    // Composition of Types

    FruitSnack {
        apple: AppleVariety::GoldenDelicious,
        banana: BananaVariety::Manzano,
        cherry: CherryVariety::Montmorency
    };

    // Simple type
    ProductCode(String::from("1234"));

    // Working with Types

    let a_person = Person { first: "Alex".to_string(), last: "Adams".to_string() };
    println!("first {:?} last {:?}", a_person.first, a_person.last);

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html
    let Person{ first, last} = a_person;
    println!("first {:?} last {:?}", first, last);

    print_quantity(OrderQuantity::UnitQuantity(10));
    print_quantity(OrderQuantity::KilogramQuantity(2.5));

    // Building a Domain Model by Composing Types
    let pay_strategy: PayInvoice = pay_invoice;
}