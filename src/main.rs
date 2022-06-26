mod config;
mod routes;
mod models;

fn main() {
    config::print_config();
    routes::health_routes::print_health_routes();
    routes::user_route::user_route_model();
    println!("Hello, world!");
}
