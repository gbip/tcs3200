#![no_std]

use nb::block;
use embedded_hal::digital::*;

let duration = 0;

enum Color {
    Red,
    Green,
    Blue,
}


//Lit la durée de d'une période de valeur opposée
fn pulseIn(SensorOut : u32, value : bool){
    let time : u32 = 0; //Chronometre

    //On attend qu'un signal arrive
    while hal::digital::InputPin::is_high(SensorOut) {}

    //On chronometre (voir hal::capture)


    return ;
}

//Retourne la couleur majoritaire
fn read_color(s0 : InputPint, s1 : InputPint, s2: InputPint, s3: InputPint, SensorOut : Hertz) -> Color {
    let couleur : Color;

    //Initialisation
    hal::digital::OutputPin::set_high(s0);
    hal::digital::OutputPin::set_low(s1);

    //Lecture
    hal::digital::OutputPin::set_low(s2);
    hal::digital::OutputPin::set_low(s3);

    duration = pulseIn(SensorOut, false);

    return couleur;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
