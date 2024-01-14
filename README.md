# Restaurant-Client

* This is restaurant client part of paidy's restaurant api task.
* The idea is to build some kind of load testing the server (application) with several kind of tasks at once
* The idea is to generate the tasks and their parameters randomly first. Then, using `rayon` to iterate over the generated tasks and call request functions in parallel
  * using `rayon`, the code will become cleaner 
* Some functions cannot be fully tested or unnecessary to test as it is already test the functionality on application server side or it relies on random


# Environment setup (Tested with Rust 1.75)

Configure the `.env` file

```
ENDPOINT = http://localhost:3333   # server endpoint
MAX_THREADS = 16                    # maximum number of clients / devices 
MAX_DELAY_TIME_IN_SECS = 0          # delay time of each generated task won't be greater than this value
MAX_REQUEST_TASKS = 100             # number of tasks to be generated  (larger -> longer time to test)
MAX_TABLES = 10                     # maximum number of tables to request
MAX_ORDERS_PER_REQUEST = 4          # maximum number of orders per request to be generated in add new orders task
MAX_ORDER_ID = 10                   # maximum order id to be sent for get specific order / remove order tasks 
```

# Usage
To run test, run restaurant application service first. 

## Unit & Integration tests
After setting `.env`, run  `cargo test` 

## Load test

To test the server with several requests simultaneously, run:

```
cargo run load 
```
   
The behaviour of load test depends on the parameters in  `.env`

## Command line application

User can run all functions (e.g., add orders, get orders for a table, get one order, and remove order) via command line.

At present, the functionality is limitted. It just sends request and shows the json result directly. 

### Add orders

```
cargo run add -t <table id> -o <food list separated by / character>
```

Currently, it does not support adding note to each order yet due to time constraint.

#### Example:

Add pizza and pasta to table 55

```
cargo run add -t 55 -o pizza/pasta
```


### Get all orders for a specific table

```
cargo run get-all -t <table id>
```


### Get a order from a table

```
cargo run get-one -t <table id> -o <order id>
```


### Remove an order from a table

```
cargo run remove -t <table id> -o <order id>
```
