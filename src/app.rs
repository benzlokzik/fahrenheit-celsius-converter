use actix_web::web;
use crate::handlers::temperature;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/convert")
            .route("/c_to_f/{temp}", web::get().to(temperature::celsius_to_fahrenheit))
            .route("/f_to_c/{temp}", web::get().to(temperature::fahrenheit_to_celsius)),
    );
}
