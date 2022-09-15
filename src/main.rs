mod config;
mod routes;
mod models;
mod ownership;
mod data;

fn main() {
    config::print_config();
    routes::health_routes::print_health_routes();
    routes::user_route::user_route_model();
    ownership::ownership::ownership();
    data::structs::data_structs();
    println!("Hello, world!");
}
