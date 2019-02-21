#![no_std]
extern crate embedded_hal as hal;


enum Color {
    Red,
    Green,
    Blue,
}

fn read_color(s0 : InputPint, s1 : InputPint, s2: InputPint, s3: InputPint, out : Hertz) -> Color {

    //Initialisation
    hal::digital::OutputPin::set_high(s0);
    hal::digital::OutputPin::set_low(s1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
