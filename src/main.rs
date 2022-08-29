use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Datelike;
use rand::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn extract_query(query_str: &str) -> HashMap<&str, &str> {
    let mut q: HashMap<&str, &str> = HashMap::new();
    let mut key: Option<&str> = None;
    for item in query_str.split("&").flat_map(|i| i.split("=")) {
        match key {
            Some(value) => {
                q.insert(value, item);
                key = None;
            }
            None => key = Some(item),
        }
    }
    q
}

#[get("/calculateDisselUsageForDistance")]
async fn calculate_dissel_usage_for_distance(req: HttpRequest) -> impl Responder {
    let query = &extract_query(req.query_string());
    let distance = match query.get("distance") {
        Some(x) => Some(x.parse::<u32>()),
        None => None,
    };
    let year_of_production = match query.get("yearOfProduction") {
        Some(x) => Some(x.parse::<u32>()),
        None => None,
    };
    let fuel_usage_per100_km = match query.get("fuelUsagePer100KM") {
        Some(x) => Some(x.parse::<u32>()),
        None => None,
    };
    if distance.is_none() || fuel_usage_per100_km.is_none() || year_of_production.is_none() {
        return HttpResponse::BadRequest().json(json!({
          "result": "Bad Request",
          "status": 400
        }));
    }
    if year_of_production.as_ref().unwrap().is_err() {
        return HttpResponse::BadRequest().json(json!({
          "result": "Please use number 'yearOfProduction'",
          "status": 400
        }));
    }
    if year_of_production.is_some() && year_of_production.as_ref().unwrap().is_ok() {
        let year = year_of_production.unwrap().unwrap();

        if year < 1886 {
            return HttpResponse::BadRequest().json(json!({
              "result": "First car was build in 1886.",
              "status": 400
            }));
        }
        let current_date = chrono::Utc::now().date();
        if year > current_date.year() as u32 {
            return HttpResponse::BadRequest().json(json!({
              "result": format!("You are from future? Actual year {}", current_date.year()),
              "status": 400
            }));
        }
    }

    if distance.is_some()
        && fuel_usage_per100_km.is_some()
        && distance.as_ref().unwrap().is_ok()
        && fuel_usage_per100_km.as_ref().unwrap().is_ok()
    {
        let result = (distance.unwrap().unwrap() * fuel_usage_per100_km.unwrap().unwrap()) / 100;
        return HttpResponse::Ok().json(json!({
          "result": {"fuelUsage": result},
          "status": 200
        }));
    }

    if distance.as_ref().unwrap().is_err() {
        return HttpResponse::BadRequest().json(json!({
          "result": "Please use natural number 'distance'",
          "status": 400
        }));
    }
    if fuel_usage_per100_km.as_ref().unwrap().is_err() {
        return HttpResponse::BadRequest().json(json!({
          "result": "Please use natural number for 'fuelUsagePer100KM'",
          "status": 400
        }));
    }

    return HttpResponse::BadRequest().json(json!({
      "result": "Bad Request",
      "status": 400
    }));
}

#[get("/probabilityOfUnitInjectorFail")]
async fn probability_of_unit_injector_fail(req: HttpRequest) -> impl Responder {
    let query = &extract_query(req.query_string());

    let vin = match query.get("VIN") {
        Some(x) => Some(x),
        None => None,
    };
    let mut rng = rand::thread_rng();
    let fail_probability = rng.gen_range(0..100);

    if vin.is_some() {
        let vin_length = vin.unwrap().to_string().graphemes(true).count();

        if vin_length > 17 {
            return HttpResponse::BadRequest().json(json!({
              "result": "To long VIN",
              "status": 400
            }));
        }

        if vin_length < 17 {
            return HttpResponse::BadRequest().json(json!({
              "result": "To short VIN",
              "status": 400
            }));
        }

        if vin_length == 17 {
            return HttpResponse::Ok().json(json!({
              "result": {"failProbability": format!("{}%", fail_probability)},
              "status": 200
            }));
        }
    }
    if vin.is_none() {
        return HttpResponse::Ok().json(json!({
          "result": {"failProbability": format!("{}%", fail_probability)},
          "status": 200
        }));
    }

    return HttpResponse::BadRequest().json(json!({
      "result": "Bad Request",
      "status": 400
    }));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_dissel_usage_for_distance)
            .service(probability_of_unit_injector_fail)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
