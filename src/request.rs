use reqwest::{blocking::{Client, Response}, header::CONTENT_TYPE, Error};
use std::{thread::sleep, time::Duration};

use crate::{gen::{TaskInfo, TaskKind}, config::Config, error::ClientError};
use rayon::prelude::*;

/// Function that converts response from reqwest to string. Then, put current thread to sleep for delay
pub fn response_to_text_with_delay( (response, delay_time) : (Response, u64)) -> Result<String, Error>{
    let result = response.text()?;
    sleep(Duration::from_secs(delay_time));
    Ok(result)
}


/// This function send add orders request to application server
pub fn request_add_orders(task: &TaskInfo) -> Result<String, ClientError>{
    let json = task.get_table_order_request_json_string().map_err(|_| ClientError::RequestError)?;
    tracing::info!("[request add orders] {}", task.endpoint_url);  
    
    // tracing::info!("{endpoint}");
    Client::new()
            .post(&task.endpoint_url)
            .header(CONTENT_TYPE, "application/json")
            .body(json)
            .send()
            .and_then(|res| Ok((res, task.delay_time)))
            .and_then(response_to_text_with_delay)
            .map_err(|_|ClientError::RequestError)

}

/// This function send get all orders (for a table) request to application server
pub fn request_get_all_orders(task: &TaskInfo) -> Result<String, ClientError>{
    tracing::info!("[request get all order] {}", task.endpoint_url);  
    
    Client::new()
            .get(&task.endpoint_url)
            .send()
            .and_then(|res| Ok((res, task.delay_time)))
            .and_then(response_to_text_with_delay)
            .map_err(|_|ClientError::RequestError)

}

/// This function send get one specific order request to application server
pub fn request_get_one_order(task: &TaskInfo) -> Result<String, ClientError>{
    tracing::info!("[request get one order] {}", task.endpoint_url);  

    Client::new()
            .get(&task.endpoint_url)
            .send()
            .and_then(|res| Ok((res, task.delay_time)))
            .and_then(response_to_text_with_delay)
            .map_err(|_|ClientError::RequestError)

}

/// This function send remove order request to application server
pub fn request_remove_order(task: &TaskInfo) -> Result<String, ClientError>{    
    tracing::info!("[request remove order] {}", task.endpoint_url);   
    
    Client::new()
            .delete(&task.endpoint_url)
            .send()
            .and_then(|res| Ok((res, task.delay_time)))
            .and_then(response_to_text_with_delay)
            .map_err(|_|ClientError::RequestError)

}


/// A wrapper function for load test
pub fn execute_tasks(tasks: &[TaskInfo]) -> Vec<Result<String, ClientError>>{
    let results: Vec<Result<String, ClientError>> = tasks.par_iter().map(|task| match task.kind{
        TaskKind::Add => request_add_orders(task),
        TaskKind::GetAll => request_get_all_orders(task),
        TaskKind::GetOne => request_get_one_order(task),
        TaskKind::Remove => request_remove_order(task),
    }).collect();

    results
}