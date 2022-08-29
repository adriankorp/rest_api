# rest_api
### Getting started

To run the project locally:

1. install **rustup** by following the [instructions](https://www.rust-lang.org/tools/install)
2. clone this repository `git clone https://github.com/adriankorp/rest_api.git`
3. to **start an API** enter project's directory and run `cargo run`. API server will run at http://127.0.0.1:8080/.

### Dependencies overview

Dependency | Description
--- | ---
[actix-web](https://actix.rs/) | Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust.
[serde_json](https://crates.io/crates/serde_json/1.0.1) | Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
[chrono](https://crates.io/crates/chrono) | Date and time utilities
[unicode-segmentation](https://crates.io/crates/unicode-segmentation) | Iterators which split strings on Grapheme Cluster or Word boundaries, according to the Unicode Standard Annex #29 rules.
[rand](https://crates.io/crates/rand) | A Rust library for random number generation.

### Available endpoints


Path | Metod | Description | Query string parameters
--- | --- | --- | ---
[/calculateDisselUsageForDistance](http://127.0.0.1:8080/calculateDisselUsageForDistance) | GET | The endpoint  return a number, which is the fuel consumption on specified distance. | distance, yearOfProduction, fuelUsagePer100KM  
[/probabilityOfUnitInjectorFail](http://127.0.0.1:8080/probabilityOfUnitInjectorFail) | GET | The endpoint return a percentage of the chance that the engine of the C6 model will fail on the Unit Injector element. | VIN


Query string parameter | Required / optional | Description | Type | Example
--- | --- | --- | --- | ---
`distance` | required | total distance between point A and point B. Provided as a natural number. Please assume that unit measurement here is KM.| unsigned integer | distance=100
`yearOfProduction` | required | year of production of the car. Provided as a number.| unsigned integer | yearOfProduction=2012
`fuelUsagePer100KM` | required | natural number that represents approximate fuel consumption per 100KM. Provided as a number.| unsigned integer | fuelUsagePer100KM=8
`VIN` | optional | not relevant, but customer really wants it here| String | VIN=VGZZZ5NZ8W031284

### Example API request

    http://127.0.0.1:8080/calculateDisselUsageForDistance?distance=10&yearOfProduction=2010&fuelUsagePer100KM=30

    
    ### Response
    
    {
    "result":{
        "fuelUsage":3
        },
    "status":200
    }