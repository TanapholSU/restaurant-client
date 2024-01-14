
use restaurant_client::config::Config;
use restaurant_client::error::ClientError;
use restaurant_client::gen::{self, TaskInfo};
use restaurant_client::request::{
    execute_tasks, request_add_orders, request_get_all_orders, request_get_one_order,
    request_remove_order,
};
use restaurant_server::model::TableOrdersRequest;

use clap::{Parser, Subcommand};
use dotenvy;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Operations,
}

#[derive(Subcommand, Debug)]
enum Operations {
    Add {
        #[arg(short, long)]
        table_id: i16,

        #[arg(short, long, value_delimiter = '/')]
        orders: Vec<String>,
    },
    GetAll {
        #[arg(short, long)]
        table_id: i16,
    },
    GetOne {
        #[arg(short, long)]
        table_id: i16,

        #[arg(short, long)]
        order_id: i32,
    },
    Remove {
        #[arg(short, long)]
        table_id: i16,

        #[arg(short, long)]
        order_id: i32,
    },
    Load
}

/// wrapper to execute add command for cli
fn execute_add_command(
    config: &Config,
    table_id: i16,
    orders: Vec<String>,
) -> Result<String, ClientError> {

    let mut table_order = TableOrdersRequest::new(table_id);
    
    orders.iter().for_each(|item| {
        table_order.add_order_wihtout_note(item);
    });

    let task = TaskInfo::new_add_task(table_id, table_order, config);
    request_add_orders(&task)
}

/// wrapper to execute get all orders from specific table command for cli
fn execute_get_all_command(config: &Config, table_id: i16) -> Result<String, ClientError> {
    let task = TaskInfo::new_get_all_task(table_id, config);
    request_get_all_orders( &task)
}

/// wrapper to execute get one order command for cli
fn execute_get_one_command(
    config: &Config,
    table_id: i16,
    order_id: i32,
) -> Result<String, ClientError> {
    let task = TaskInfo::new_get_one_task(table_id, order_id, config);
    request_get_one_order( &task)
}

/// wrapper to execute remove order command for cli
fn execute_remove_command(
    config: &Config,
    table_id: i16,
    order_id: i32,
) -> Result<String, ClientError> {
    let task = TaskInfo::new_remove_task(table_id, order_id, config);
    request_remove_order( &task)
}

fn main() {
    tracing_subscriber::fmt().with_thread_names(true).init();
    // load env
    dotenvy::dotenv().ok();
    let mut config: Config =
        envy::from_env::<Config>().expect("Cannot load config from env (exit now)");

    // set delay to 0 because it is unncessary
    config.set_max_delay_time_to_zero();

    let c = Cli::parse();
    let results = match c.command {
        Operations::Add { table_id, orders } => execute_add_command(&config, table_id, orders),
        Operations::GetAll { table_id } => execute_get_all_command(&config, table_id),
        Operations::GetOne { table_id, order_id } => execute_get_one_command(&config, table_id, order_id),
        Operations::Remove { table_id, order_id } => execute_remove_command(&config, table_id, order_id),
        Operations::Load => load_test(&config)
    };

    let output = results.unwrap_or_else(|err| format!("Error {err}"));
    println!("{output}");
}

fn load_test(config: &Config) -> Result<String, ClientError>{
    tracing::info!("{config:?}");
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(config.get_max_threads() as usize)
        .build_global();
    let tasks = gen::gen_all_tasks(&config);
    let results = execute_tasks(&tasks);
    let success = results.iter().filter(|x| x.is_ok()).count();
    Ok(format!("Loading test result -> SUCCESS:{} / FAILED: {}", success, results.len() - success))
}
