use rand::{self, Rng};
use restaurant_server::model::TableOrdersRequest;

use crate::{config::Config, error::ClientError};


/// We can categorize task for each server function (i.e., Add orders, Get one order, Get All order (of a table) and Remove order)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskKind{
    Add,
    GetOne,
    GetAll,
    Remove
}


impl TaskKind{
    /// function to get TaskKind base on the input random index. Used for testing
    pub fn select(index: u8) -> TaskKind{
        match index % 4{
            0 => TaskKind::Add,
            1 => TaskKind::GetOne,
            2 => TaskKind::GetAll,
            3 => TaskKind::Remove,
            _ => TaskKind::Add
        }
    }
}

/// Task info stores 1.) what kind of task and the parameters it need to execute the task
#[derive(Debug)]
pub struct TaskInfo{
    /// task category
    pub kind: TaskKind,

    /// target table id 
    pub table_id: i16,

    /// target order id (for get one order and remove order function)
    pub order_id: Option<i32>,

    /// table order request object for add operation
    pub table_orders_request: Option<TableOrdersRequest>,

    pub endpoint_url: String,

    pub delay_time: u64
}

impl TaskInfo{
    /// init function to generate add task. the delay time and endpoint url are determined here.
    pub fn new_add_task(table_id: i16, table_orders: TableOrdersRequest, config: &Config) -> Self{
        let delay_time = rand::thread_rng().gen_range(0..=config.get_max_delay_time_in_secs());
        let endpoint_url = format!("{}/api/v1/tables/{}/orders", config.get_endpoint(), table_id);
        Self { kind: TaskKind::Add, table_id, order_id: None, table_orders_request: Some(table_orders), endpoint_url, delay_time }
    }

    /// init function to generate get all orders from specific table task. The delay time and endpoint url are determined here.
    pub fn new_get_all_task( table_id: i16, config: &Config) -> Self{
        let delay_time = rand::thread_rng().gen_range(0..=config.get_max_delay_time_in_secs());
        let endpoint = format!("{}/api/v1/tables/{}/orders", config.get_endpoint(), table_id);
        
        Self {  kind: TaskKind::GetAll, table_id, order_id: None, table_orders_request: None, endpoint_url: endpoint, delay_time }
    }

    
    /// init function to generate get specific order task. The delay time and endpoint url are determined here.
    pub fn new_get_one_task(table_id: i16, order_id: i32, config: &Config) -> Self{
        let delay_time = rand::thread_rng().gen_range(0..=config.get_max_delay_time_in_secs());
        let endpoint = format!("{}/api/v1/tables/{}/orders/{}", config.get_endpoint(), table_id, order_id);
        
        Self {  kind: TaskKind::GetOne, table_id, order_id: Some(order_id), table_orders_request: None, endpoint_url: endpoint, delay_time }
    }

    /// utility function to generate order removal task. The delay time and endpoint url are determined here.
    pub fn new_remove_task(table_id: i16, order_id: i32, config: &Config) -> Self{
        let delay_time = rand::thread_rng().gen_range(0..=config.get_max_delay_time_in_secs());
        let endpoint = format!("{}/api/v1/tables/{}/orders/{}", config.get_endpoint(), table_id, order_id);
        
        Self {  kind: TaskKind::Remove, table_id, order_id: Some(order_id), table_orders_request: None, endpoint_url: endpoint, delay_time }
    }


    /// utility function to get json request from table order request (for add order operation) 
    pub fn get_table_order_request_json_string(&self) -> Result<String, ClientError>{
        self.table_orders_request.as_ref()
            .ok_or_else(|| ClientError::SerializationError)
            .and_then(|order| order.to_json().map_err(|x| ClientError::SerializationError))
    }
}


/// function to generate one add TableOrderRequest (add new orders) task
fn gen_add_orders_task(config: &Config) -> TaskInfo{
    let table_id = rand::thread_rng().gen_range(1..=config.get_max_tables());
    let num_orders = rand::thread_rng().gen_range(1..=config.get_max_orders_per_request());
    let mut order = TableOrdersRequest::new(table_id);

    // more cleaner than for_each
    for i in 0..num_orders{
        order.add_order(format!("item-{table_id}-{i}").as_str(), format!("note-{table_id}-{i}").as_str());
    };

    TaskInfo::new_add_task( table_id, order, config)
}

/// function to generate one add TableOrderRequest (add new orders) task
fn gen_remove_order_task(config: &Config) -> TaskInfo{
    let order_id = rand::thread_rng().gen_range(1..=config.get_max_order_id());
    TaskInfo::new_remove_task( 1, order_id, config)
}


/// function to generate one add TableOrderRequest (add new orders) task
fn gen_get_all_orders_task(config: &Config) -> TaskInfo{
    let table_id = rand::thread_rng().gen_range(1..=config.get_max_tables());
    TaskInfo::new_get_all_task( table_id, config)
}

/// function to generate one add TableOrderRequest (add new orders) task
fn gen_get_one_order_task(config: &Config) -> TaskInfo{
    let table_id = rand::thread_rng().gen_range(1..=config.get_max_tables());
    let order_id = rand::thread_rng().gen_range(1..=config.get_max_order_id());
    TaskInfo::new_get_one_task( table_id, order_id, config)
}



/// randomly generate single task and its necessary parameters
fn gen_single_task(config: &Config) -> TaskInfo {
    let mut rng = rand::thread_rng();
    match TaskKind::select(rng.gen_range(0..4)){
        TaskKind::Add => gen_add_orders_task(config),
        TaskKind::GetOne => gen_get_one_order_task(config),
        TaskKind::GetAll => gen_get_all_orders_task(config),
        TaskKind::Remove => gen_remove_order_task(config),
    }
}

/// function generates random tasks 
pub fn gen_all_tasks(config: &Config) -> Vec<TaskInfo>{
    (0..config.get_max_request_tasks()).map(|id| gen_single_task(config)).collect()
}


#[cfg(test)]
mod test{
    use crate::config::Config;

    use super::TaskKind;
    use super::*;

    #[test]
    fn test_select_task_kind(){
        let results : Vec<TaskKind> = (0..8).map(TaskKind::select).collect();
        
        assert_eq!(results[0], TaskKind::Add);
        assert_eq!(results[1], TaskKind::GetOne);
        assert_eq!(results[2], TaskKind::GetAll);
        assert_eq!(results[3], TaskKind::Remove);

        assert_eq!(results[4], TaskKind::Add);
        assert_eq!(results[5], TaskKind::GetOne);
        assert_eq!(results[6], TaskKind::GetAll);
        assert_eq!(results[7], TaskKind::Remove);

    }

    #[test]
    fn test_gen_add_order_task(){
        let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
        let result = gen_add_orders_task(&config);
        
        assert_eq!(result.kind, TaskKind::Add);
        assert!((0..config.get_max_tables()).contains(&result.table_id));

        let order = result.table_orders_request.unwrap();
        assert_eq!(order.table_id, result.table_id);
        
        assert!(order.orders.len() > 0);
        assert!(order.orders.len() < config.get_max_orders_per_request() as usize);

    }


    #[test]
    fn test_gen_get_all_task(){
        let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
        let result = gen_get_all_orders_task(&config);
        
        assert_eq!(result.kind, TaskKind::GetAll);
        assert!((0..config.get_max_tables()).contains(&result.table_id));
    }

    

    #[test]
    fn test_get_one_task(){
        let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
        let result = gen_get_one_order_task(&config);
        
        assert_eq!(result.kind, TaskKind::GetOne);
        assert!((0..config.get_max_tables()).contains(&result.table_id));
        assert!((0..config.get_max_order_id()).contains(&result.order_id.unwrap()));
        
    }

    
    #[test]
    fn test_remove_task(){
        let config = envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
        let result = gen_remove_order_task(&config);
        
        assert_eq!(result.kind, TaskKind::Remove);
        assert!((0..config.get_max_tables()).contains(&result.table_id));
        assert!((0..config.get_max_order_id()).contains(&result.order_id.unwrap()));
        
    }
}