use actix_cors::Cors;
use actix_web::get;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rppal::gpio::{Gpio, Level};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::{result, thread}; // Add the missing import for the `get` attribute

const GPIO_LED1: u8 = 22;
const GPIO_LED2: u8 = 23;
const GPIO_LED3: u8 = 24;
const GPIO_LED4: u8 = 25;
const GPIO_LED5: u8 = 04;
const GPIO_LED6: u8 = 21;
const GPIO_LED7: u8 = 20;
const GPIO_LED8: u8 = 16;
const LED_GPIO: [u8; 8] = [
    GPIO_LED1, GPIO_LED2, GPIO_LED3, GPIO_LED4, GPIO_LED5, GPIO_LED6, GPIO_LED7, GPIO_LED8,
];

#[derive(Deserialize)]
struct PinControl {
    pin_index: usize,
    value: bool,
}

async fn control_pin(info: web::Json<PinControl>) -> impl Responder {
    println!("Updating pin state");
    tokio::task::spawn_blocking(move || {
        set_pin(info.pin_index, info.value);
    })
    .await
    .unwrap(); // Handle this unwrap more gracefully in production code

    "Pin state updated"
}

async fn greet(req: HttpRequest) -> &'static str {
    let name = req.match_info().get("name").unwrap_or("World");
    if (name == "On") {
        test(true);
    } else if (name == "Off") {
        test(false);
    }
    "lol"
}

// fn set_pin(pin_index: usize, value: bool) {
//     if pin_index < LED_GPIO.len() {
//         println!("Setting pin {} to {}", pin_index, value);
//         let pin_number = LED_GPIO[pin_index] as u8; // Cast pin_number to u8
//         let mut pin = Gpio::new().unwrap().get(pin_number).unwrap().into_output();
//         // let mut pin = Gpio::new().unwrap().get(pin_number).unwrap().into_output();
//         pin.set_reset_on_drop(false);
//         if value {
//             pin.set_high();
//         } else {
//             pin.set_low();
//         }
//     } else {
//         // Handle the case where the pin_index is out of bounds
//         println!("Pin index {} is out of bounds", pin_index);
//     }
// }
fn set_pin(pin_index: usize, value: bool) {
    if pin_index < LED_GPIO.len() {
        println!("Setting pin {} to {}", pin_index, value);
        let pin_number = LED_GPIO[pin_index] as u8;

        match Gpio::new() {
            Ok(gpio) => match gpio.get(pin_number) {
                Ok(pin) => {
                    let mut pin = pin.into_output();
                    pin.set_reset_on_drop(false);
                    if value {
                        pin.set_high();
                    } else {
                        pin.set_low();
                    }
                }
                Err(e) => println!("Failed to get pin {}: {}", pin_number, e),
            },
            Err(e) => println!("Failed to initialize GPIO: {}", e),
        }
    } else {
        println!("Pin index {} is out of bounds", pin_index);
    }
}

#[derive(Serialize)]
struct PinState {
    pin_index: usize,
    state: bool,
}

async fn read_pin_handler(path: web::Path<usize>) -> HttpResponse {
    let pin_index = path.into_inner();

    let pin_state = tokio::task::spawn_blocking(move || read_pin(pin_index))
        .await
        .unwrap(); // Consider handling errors more gracefully

    HttpResponse::Ok().json(PinState {
        pin_index,
        state: pin_state,
    })
}
fn read_pin(pin_index: usize) -> bool {
    if pin_index < LED_GPIO.len() {
        let pin_number = LED_GPIO[pin_index] as u8; // Cast pin_number to u8
        let pin = Gpio::new().unwrap().get(pin_number).unwrap().into_input();
        pin.read() == Level::High
    } else {
        // Handle the case where the pin_index is out of bounds
        println!("Pin index {} is out of bounds", pin_index);
        false
    }
}

fn test(new_led_value: bool) {
    // Retrieve the GPIO pin and configure it as an output.
    let mut pin1 = Gpio::new().unwrap().get(GPIO_LED1).unwrap().into_output();
    let mut pin2 = Gpio::new().unwrap().get(GPIO_LED2).unwrap().into_output();
    let mut pin3 = Gpio::new().unwrap().get(GPIO_LED3).unwrap().into_output();
    let mut pin4 = Gpio::new().unwrap().get(GPIO_LED4).unwrap().into_output();
    let mut pin5 = Gpio::new().unwrap().get(GPIO_LED5).unwrap().into_output();
    let mut pin6 = Gpio::new().unwrap().get(GPIO_LED6).unwrap().into_output();
    let mut pin7 = Gpio::new().unwrap().get(GPIO_LED7).unwrap().into_output();
    let mut pin8 = Gpio::new().unwrap().get(GPIO_LED8).unwrap().into_output();

    pin1.set_reset_on_drop((false));
    pin2.set_reset_on_drop((false));
    pin3.set_reset_on_drop((false));
    pin4.set_reset_on_drop((false));
    pin5.set_reset_on_drop((false));
    pin6.set_reset_on_drop((false));
    pin7.set_reset_on_drop((false));
    pin8.set_reset_on_drop((false));

    if (new_led_value == true) {
        pin1.set_high();
        pin2.set_high();
        pin3.set_high();
        pin4.set_high();
        pin5.set_high();
        pin6.set_high();
        pin7.set_high();
        pin8.set_high();
    } else {
        pin1.set_low();
        pin2.set_low();
        pin3.set_low();
        pin4.set_low();
        pin5.set_low();
        pin6.set_low();
        pin7.set_low();
        pin8.set_low();
    }
}
async fn not_found() -> impl Responder {
    "dawd"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello World server started");
    HttpServer::new(|| {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors) // Apply CORS middleware to the app
            .service(
                web::scope("")
                    .route("", web::get().to(greet))
                    .route("{name}", web::get().to(greet))
                    .route("control_pin", web::post().to(control_pin))
                    .route("/read_pin/{pin_index}", web::get().to(read_pin_handler)), // New route for reading pin state
            )
            .default_service(web::to(not_found))
    })
    .bind("[::]:8080")?
    .run()
    .await
}
