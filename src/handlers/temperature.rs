use actix_web::{web, HttpResponse, Responder};
use crate::models::temperature_conversion::{Celsius, Fahrenheit};

pub async fn celsius_to_fahrenheit(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let fahrenheit = Celsius::new(temp).to_fahrenheit();
    HttpResponse::Ok().json(fahrenheit)
}

pub async fn fahrenheit_to_celsius(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let celsius = Fahrenheit::new(temp).to_celsius();
    HttpResponse::Ok().json(celsius)
}
