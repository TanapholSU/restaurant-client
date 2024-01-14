use serde::{Deserialize};
#[derive( Debug, Deserialize, Clone)]

/// This struct stores parsed configuration from env
pub struct Config{

    /// Target URL endpoint for load testing. 
    endpoint: Option<String>,
    
    /// Number of threads (staffs/clients) to operate the service
    max_threads: Option<i32>,

    /// Delay time (seconds) for each request
    max_delay_time_in_secs: Option<u64>,

    /// maximum request tasks to generate
    max_request_tasks: Option<i32>,

    /// Maximum number of available tables
    max_tables: Option<i16>,

    /// Maximum orders per add orders request (max is u8)
    max_orders_per_request: Option<u8>,

    /// Maximum order id for geting specific order and remove order request 
    max_order_id: Option<i32>
}



impl Config{
    pub fn set_max_delay_time_to_zero(&mut self){
        self.max_delay_time_in_secs = None
    }

    /// utilities functions to get endpoint (if exists in config). Otherwise, it returns default value `http://localhost:3000`
    pub fn get_endpoint(&self) -> String{
        self.endpoint.clone().unwrap_or("http://localhost:3000".to_string())
    }

    /// utilities functions to get max threads (if exists in config). Otherwise, default value `8` is returned
    pub fn get_max_threads(&self) -> i32{
        self.max_threads.unwrap_or(8)
    }

    /// utilities functions to get max orders (if exists in config). Otherwise, default value `10` is returned
    pub fn get_max_orders_per_request(&self) -> u8{
        self.max_orders_per_request.unwrap_or(10)
    }

    /// utilities functions to get delay time (if exists in config). Otherwise, default value `1` is returned
    pub fn get_max_delay_time_in_secs(&self) -> u64{
        self.max_delay_time_in_secs.unwrap_or(0)
    }
    
    /// utilities functions to get duration (if exists in config). Otherwise, default value `1` is returned
    pub fn get_max_request_tasks(&self) -> i32{
        self.max_request_tasks.unwrap_or(100)
    }

    /// utilities functions to max tables (if exists in config). Otherwise, default value `100` is returned
    pub fn get_max_tables(&self) -> i16{
        self.max_tables.unwrap_or(100)
    }
    
    /// utilities functions to max order id (if exists in config). Otherwise, default value `100` is returned
    pub fn get_max_order_id(&self) -> i32{
        self.max_order_id.unwrap_or(100)
    }
}