#[cfg(test)]

// -- ----------------------------------------------------------------- TESTS UTILS -- //
// -- ----------------------------------------------------------------- ----------- -- //

mod test_orderbook_utils {

    use atelier::data::market::{Side, Level, Orderbook};
    use rand::distributions::Uniform;
    use rand::{thread_rng, Rng};
    
    // ------------------------------------------------------------- TEST ORDERBOOK -- //

    pub fn test_orderbook() -> Orderbook {
        Orderbook::random()
    }

    // ----------------------------------------------------------------- TEST LEVEL -- //

    pub fn test_level(testable_ob: &Orderbook) -> Result<&Level, ()> {

        // Random samples
        let mut uni_rand = thread_rng();
        let rand_level_b = uni_rand.sample(Uniform::new(0, testable_ob.bids.len()));
        let rand_level_a = uni_rand.sample(Uniform::new(0, testable_ob.asks.len()));

        // Get a random Level from the Test Orderbook
        let random_level = match Side::random() {
            Side::Bids => &testable_ob.bids[rand_level_b],
            Side::Asks => &testable_ob.asks[rand_level_a]
        };

        Ok(random_level)
    }
}

// -- ------------------------------------------------------------- ORDERBOOK TESTS -- //
// -- ------------------------------------------------------------- --------------- -- //

mod tests {

    // ------------------------------------------------------------------ FIND_LEVEL -- /
    // ------------------------------------------------------------------ ---------- -- /

    // ---------------------------------------------------- FIND_LEVEL: OUTPUT VALUE -- /
    
    #[test]
    fn find_level_output_value() {
        use super::*;
        use test_orderbook_utils::*;
        
        // Get a random Orderbook from test_orderbook
        let testable_ob = test_orderbook();
        // Get a random Level from the test_level
        let random_level = test_level(&testable_ob).unwrap();

        // Find the same Level using the function
        let level_price: f64 = random_level.price;
        let level_found = testable_ob.find_level(&level_price);

        println!("randomly picked Level.level_id: {:?}", random_level.level_id);
        println!("randomly picked Level.side: {:?}", random_level.side);
        println!("randomly picked Level.price: {:?}\n", random_level.price);

        if level_found.as_ref().unwrap() > &1 {
            let a_level = level_found.as_ref().unwrap().abs() - 1;
            println!("Ask level found: {:?}", a_level + 1);
            println!("Ask level price: {:?}", testable_ob.asks[a_level as usize].price);
            
            // Level found is the same as the randomly extracted one
            assert_eq!(testable_ob.asks[a_level as usize], *random_level);
        }

        else if level_found.as_ref().unwrap() < &1 {
            let b_level = level_found.as_ref().unwrap().abs() - 1;
            println!("Bid level found: {:?}", b_level + 1);
            println!("Bid level price: {:?}", testable_ob.bids[b_level as usize].price);
            
            // Level found is the same as the randomly extracted one
            assert_eq!(testable_ob.bids[b_level as usize], *random_level);
        }

    }

    // -------------------------------------------------------------- RETRIEVE_LEVEL -- /
    // -------------------------------------------------------------- -------------- -- /
    
    // ------------------------------------------------ RETRIEVE_LEVEL: OUTPUT VALUE -- /
    
    #[test]
    fn retrieve_level_output_value() {
        use super::*;
        use test_orderbook_utils::*;
        
        // Get a random Orderbook from test_orderbook
        let testable_ob = test_orderbook();
        // Get a random Level from the test_level
        let random_level = test_level(&testable_ob).unwrap();

        // Find the same Level using the function
        let level_retrieved = testable_ob.retrieve_level(&random_level.price);

        println!("\nlevel retrieved:\n\n{:?}", level_retrieved);

        // Level retrieved
        assert!(level_retrieved.is_ok());

    }

    // ---------------------------------------------------------------- DELETE_LEVEL -- /
    // ---------------------------------------------------------------- ------------ -- /

    // -------------------------------------------------- DELETE_LEVEL: OUTPUT VALUE -- /
    
    #[test]
    fn delete_level_output_value() {
        use super::*;
        use test_orderbook_utils::*;

        // Get a random Orderbook from test_orderbook
        let mut testable_ob_original = test_orderbook();
        let testable_ob_cloned = test_orderbook().clone();
        
        // Get a random Level from the test_level
        let level_to_delete = test_level(&testable_ob_cloned).unwrap();
        println!("Price of Level to be deleted: {:?}", level_to_delete.price);
        
        let _ = testable_ob_original.delete_level(&level_to_delete.price);

        match testable_ob_cloned.find_level(&level_to_delete.price) {
            Ok(_) =>  {
                println!("Level founded, level_id: {:?}, side: {:?}, price: {:?}",
                    &level_to_delete.level_id,
                    &level_to_delete.side,
                    &level_to_delete.price)
            },
            Err(_) => {
                println!("Level not founded")
            }
        };
        
        let post_d = match testable_ob_original.find_level(&level_to_delete.price) {
            Ok(_) => "Level founded",
            Err(_) => "Level not founded"
        };

        println!("As a result: {:?}", post_d);
    }

    // ---------------------------------------------------------------- INSERT_LEVEL -- /
    // ---------------------------------------------------------------- ------------ -- /

    // -------------------------------------------------- INSERT_LEVEL: OUTPUT VALUE -- /
    
    // #[test]
    // fn insert_level_output_value() {
    // }

    // ------------------------------------------------------------------ FIND_ORDER -- /
    // ------------------------------------------------------------------ ---------- -- /

    // ---------------------------------------------------- FIND_ORDER: OUTPUT VALUE -- /
    
    // #[test]
    // fn find_order_output_value() {
    // }

    // -------------------------------------------------------------- RETRIEVE_ORDER -- /
    // -------------------------------------------------------------- -------------- -- /

    // ------------------------------------------------ RETRIEVE_ORDER: OUTPUT VALUE -- /
    
    // #[test]
    // fn retrieve_order_output_value() {
    // }

    // ---------------------------------------------------------------- DELETE_ORDER -- /
    // ---------------------------------------------------------------- ------------ -- /

    // -------------------------------------------------- DELETE_ORDER: OUTPUT VALUE -- /
    
    // #[test]
    // fn delete_order_output_value() {
    // }

    // ---------------------------------------------------------------- INSERT_ORDER -- /
    // ---------------------------------------------------------------- ------------ -- /
    
    // -------------------------------------------------- INSERT_ORDER: OUTPUT VALUE -- /

    // #[test]
    // fn insert_order_output_value() {
    // }

    // ---------------------------------------------------------------- MODIFY_ORDER -- /
    // ---------------------------------------------------------------- ------------ -- /

    // -------------------------------------------------- MODIFY_ORDER: OUTPUT VALUE -- /
    
    // #[test]
    // fn modify_order_output_value() {
    // }

}

