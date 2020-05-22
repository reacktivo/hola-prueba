extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello World!")))
        println!("Universidad Politecnica de San Luis potosi");
        println!("Isaac Amalio Perez Macareno 180672");
        println!("Chávez Flores Rebeca 171176");
        println!("Cruz Becerra Rodrigo Emmanuel 170736");
    }

    let _server = Iron::new(hello_world).http("0.0.0.0:8080").unwrap();
    println!("On 8080");
}
