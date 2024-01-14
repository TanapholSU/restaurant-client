use restaurant_server::model::TableOrdersRequest;
use serde_json::{self, Value};
use restaurant_client::{request::*, gen::TaskInfo, config::Config};


#[allow(unused)]
#[test]
fn test_request_add_orders(){
    dotenvy::dotenv();
    let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
    
    let mut order = TableOrdersRequest::new(1);
    order.add_order("item-1", "note1");
    order.add_order("item-2", "note2");

    let task = TaskInfo::new_add_task( 1, order, &config);
    let results = request_add_orders(&task).unwrap();

    let json: Value = serde_json::from_str(&results).unwrap();
    
    // check whether the result is TableOrderResponse object (table id and orders exist or not)
    // the correctness check is skipped (already have tests in the server)

    let table_id = json.pointer("/table_id").unwrap();
    assert_eq!(table_id.as_i64().unwrap(), 1);

    let orders = json.pointer("/orders").unwrap();
    assert!(orders.as_array().unwrap().len() > 0);    
}

#[allow(unused)]
#[test]
fn test_request_get_all_orders(){
    dotenvy::dotenv();
    let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");

    let task = TaskInfo::new_get_all_task( 1, &config);
    let results = request_get_all_orders( &task).unwrap();

    let json: Value = serde_json::from_str(&results).unwrap();

    // check whether the result is TableOrderResponse object (table id and orders exist or not)
    // the correctness check is skipped (already have tests in the server)
    let table_id = json.pointer("/table_id").unwrap();
    assert_eq!(table_id.as_i64().unwrap(), 1);

    let orders = json.pointer("/orders").unwrap();
    assert!(orders.as_array().unwrap().len() > 0);    
    
}

#[test]
#[allow(unused)]

fn test_request_get_one_order(){
    dotenvy::dotenv();
    let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");

    // ensure that order does exist in table 1
    test_request_add_orders();

    
    let task = TaskInfo::new_get_all_task( 1, &config);
    let results = request_get_all_orders(&task).unwrap();
    let json_value: Value = serde_json::from_str(&results).unwrap();

    let existing_order_id = json_value.pointer("/orders/0/order_id")
                                                .unwrap()
                                                .as_i64().unwrap();


    let task = TaskInfo::new_get_one_task( 1, existing_order_id as i32, &config);
    let results = request_get_one_order(&task).unwrap();

    let json: Value = serde_json::from_str(&results).unwrap();

    // check whether the result is TableOrderResponse object (table id and orders exist or not)
    // the correctness check is skipped (already have tests in the server)
    let table_id = json.pointer("/table_id").unwrap();
    assert_eq!(table_id.as_i64().unwrap(), 1);

    let orders = json.pointer("/orders").unwrap();
    assert!(orders.as_array().unwrap().len() == 1);    
    
    
}


#[test]
#[allow(unused)]

fn test_request_remove_one_order(){
    dotenvy::dotenv();
    let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");

    // ensure that order does exist in table 1
    test_request_add_orders();

    
    let task = TaskInfo::new_get_all_task( 1, &config);
    let results = request_get_all_orders( &task).unwrap();
    let json_value: Value = serde_json::from_str(&results).unwrap();

    let existing_order_id = json_value.pointer("/orders/0/order_id")
                                                .unwrap()
                                                .as_i64().unwrap();


    let task = TaskInfo::new_remove_task( 1, existing_order_id as i32, &config);
    request_remove_order( &task);
                                            


    let task = TaskInfo::new_get_one_task( 1, existing_order_id as i32, &config);
    let results = request_get_one_order( &task).unwrap();
    
}
