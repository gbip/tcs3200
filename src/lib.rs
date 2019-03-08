#![no_std]

//use embedded_hal::digital::*;
use embedded_hal::prelude::*;
use embedded_hal::digital::OutputPin;
//use nb::*;

//Bibliothèque pour utiliser un capteur couleur tcs3200
//
//
//     Capteur de couleur TCS3200 Vu de dessus
//
//     --------------------------
//     |                        |
//     |   RGB Color Sensor     |
//     |                        |
//     |     O          O       |
//     |                        |
//     |                        |
//     |     O          O       |
//     |                        |
//     |                        |
//     |                        |
//     --------------------------
//       |  |  |  |  |  |  |  |
//       |  |  |  |  |  |  |  |
//       S3 S2 S1 S0 L  O  G  V
//                   e  u  N  C
//                   d  t  D  C
//
//
//
//

///Contient les 3 composantes d'une couleur échantillonées de 0 (noir) à 255 (blanc)
pub struct Color {
    pub Red : u32, // de 0 à 255
    pub Green : u32,
    pub Blue : u32,
}

pub struct Hertz(u32);


//La fonction hzToColor reçoit en arguments :
//      fnRead --> Fonction de lecture de la fréquence (retourne des Hertz)
// Test tout les filtres et retient le plus lumineux
///La fonction hzToColor reçoit en argument les 4 pins de contrôle du capteur couleur ainsi qu'une fonction de lecture de la fréquence. Elle retourne un struct Color.
pub fn hzToColor<S1,S2,S3,S4>(s0 : &mut S1, s1 : &mut S2, s2: &mut S3, s3: &mut S4, fnRead: &Fn() -> u32) -> Color
    where
        S1 : OutputPin,
        S2 : OutputPin,
        S3 : OutputPin,
        S4 : OutputPin
{
    let mut color : Color = Color {
        Red: 0,
        Green: 0,
        Blue: 0
    };
    let mut freqMax : u32;

    color.Green = 0;
    color.Blue = 0;
    color.Red = 0;

    s0.set_high();
    s1.set_high();

    //On lit les blancs
    s2.set_high();
    s3.set_low();
    freqMax = fnRead();

    //On lis le vert
    s2.set_high();
    s3.set_high();
    color.Green = ((fnRead() as f32/freqMax as f32)*255.0) as u32 ;

    //On lis le bleu
    s2.set_low();
    s3.set_high();
    color.Blue = ((fnRead() as f32/freqMax as f32)*255.0) as u32;

    //On lis le rouge
    s2.set_low();
    s3.set_low();
    color.Red = ((fnRead() as f32/freqMax as f32)*255.0) as u32;

    s0.set_low();
    s1.set_low();



    color
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}