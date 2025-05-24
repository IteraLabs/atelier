#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_order_utils {

    use std::time::{SystemTime, UNIX_EPOCH};

    // -------------------------------------------------------------- TEST TIMESTAMP --//

    pub fn test_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64
    }
}

// -- ----------------------------------------------------------------- ORDER TESTS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod tests {

    // --------------------------------------------------------------------- RANDOM -- //
    // --------------------------------------------------------------------- ------ -- //

    // -------------------------------------------------------- RANDOM: OUTPUT TYPE -- //

    #[test]
    fn order_random_output_type() {
        use atelier_core::orders::{Order, OrderSide, OrderType};

        let i_order: Order = Order::random(
            OrderType::random(),
            OrderSide::random(),
            (10_000.00, 11_000.00),
            (0.0, 0.1),
        )
        .unwrap();

        assert!(matches!(i_order, _), "Expected Order type");
    }

    // ------------------------------------------------------------------- ORDER_ID -- //
    // --------------------------------------------------------------------- ------ -- //

    // ----------------------------------------------------- ORDER_ID: OUTPUT VALUE -- //

    #[test]
    fn order_id_output_value() {
        use crate::test_order_utils::*;
        use atelier_core::orders::{Order, OrderSide, OrderType};

        let mut failures = Vec::new();

        let r_order_type = OrderType::random();
        let r_order_side = OrderSide::random();

        let i_order: Order = Order::random(
            r_order_type,
            r_order_side,
            (10_000.00, 11_000.00),
            (0.0, 0.1),
        )
        .unwrap();

        let r_order_ts = test_timestamp();
        let decoded_order_id = Order::decode_order_id(i_order.order_id);

        if decoded_order_id.0 != r_order_side {
            failures.push("order_id.0 != r_order_side");
        }

        if decoded_order_id.1 != r_order_type {
            failures.push("order_id.1 != r_order_type");
        }

        if decoded_order_id.2 >= r_order_ts as u64 {
            failures.push("order_id.2 != r_order_ts");
        }

        assert!(
            failures.is_empty(),
            "Test failed with: \n{}",
            failures.join("\n")
        )
    }
}
