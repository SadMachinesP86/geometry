extern crate magnus;

use magnus::{Error};

mod rectangle;
mod circle;
mod integer;

#[magnus::init]
fn init() -> Result<(), Error> {
    circle::expose().map_err(|err| println!("{:?}", err)).ok();
    rectangle::expose().map_err(|err| println!("{:?}", err)).ok();
    integer::expose().map_err(|err| println!("{:?}", err)).ok();
    Ok(())
}
