use actix_web::{web, HttpResponse, Responder};

// Handler for converting Celsius to Fahrenheit
pub async fn celsius_to_fahrenheit(celsius: web::Path<f64>) -> impl Responder {
    let celsius = celsius.into_inner();
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    HttpResponse::Ok().json(fahrenheit)
}

// Handler for converting Fahrenheit to Celsius
pub async fn fahrenheit_to_celsius(fahrenheit: web::Path<f64>) -> impl Responder {
    let fahrenheit = fahrenheit.into_inner();
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    HttpResponse::Ok().json(celsius)
}
