#![feature(never_type)]
type Undefined = !;

struct CustomerInfo(Undefined);
struct ShippingAddress(Undefined);
struct BillingAddress(Undefined);
struct OrderLine(Undefined);
struct BillingAmount(Undefined);

struct Order {
    customer_info: CustomerInfo,
    shipping_address: ShippingAddress,
    billing_address: BillingAddress,
    order_lines: Vec<OrderLine>,
    amount_to_bill: BillingAmount,
}

fn main() {}

