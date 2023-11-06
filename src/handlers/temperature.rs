use actix_web::{web, HttpResponse, Responder};
use crate::models::temperature_conversion::{Celsius, Fahrenheit, Kelvin};

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

pub async fn kelvin_to_celsius(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let celsius = Kelvin::new(temp).to_celsius();
    HttpResponse::Ok().json(celsius)
}

pub async fn celsius_to_kelvin(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let kelvin = Celsius::new(temp).to_kelvin();
    HttpResponse::Ok().json(kelvin)
}

pub async fn kelvin_to_fahrenheit(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let fahrenheit = Kelvin::new(temp).to_fahrenheit();
    HttpResponse::Ok().json(fahrenheit)
}

pub async fn fahrenheit_to_kelvin(temp: web::Path<f64>) -> impl Responder {
    let temp = temp.into_inner();
    let kelvin = Fahrenheit::new(temp).to_kelvin();
    HttpResponse::Ok().json(kelvin)
}
