fn main() {
    tracing_subscriber::fmt().with_thread_names(true).init();
    // load env
    dotenvy::dotenv().ok();
    let mut config: Config =
        envy::from_env::<Config>().expect("Cannot load config from env (exit now)");
    println!("Hello, world!");
}
