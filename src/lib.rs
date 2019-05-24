#![no_std]

//use embedded_hal::digital::*;
//use embedded_hal::prelude::*;
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
    pub red : u32, // de 0 à 255
    pub green : u32,
    pub blue : u32,
}

pub struct Hertz(u32);


//La fonction hzToColor reçoit en arguments :
//      fn_read --> Fonction de lecture de la fréquence (retourne des Hertz)
// Test tout les filtres et retient le plus lumineux
///La fonction hzToColor reçoit en argument les 4 pins de contrôle du capteur couleur, une fonction de lecture de la fréquence ainsi que le nombre de lectures souhaité.
///Elle retourne un struct color.
pub fn hz_to_color<S1,S2,S3,S4>(s0 : &mut S1, s1 : &mut S2, s2: &mut S3, s3: &mut S4, fn_read: &Fn() -> u32, nb_pass : u32) -> Color
    where
        S1 : OutputPin,
        S2 : OutputPin,
        S3 : OutputPin,
        S4 : OutputPin
{
    let mut couleur : Color = Color {
        red: 0,
        green: 0,
        blue: 0
    };
    let mut freq_max : u32;


    couleur.green = 0;
    couleur.blue = 0;
    couleur.red = 0;


    for _i in 1..nb_pass{

        s0.set_high();
        s1.set_high();

        //On lit les blancs
        s2.set_high();
        s3.set_low();
        freq_max = fn_read();

        //On lis le vert
        s2.set_high();
        s3.set_high();
        couleur.green += ((fn_read() as f32/freq_max as f32)*255.0) as u32;

        //On lis le bleu
        s2.set_low();
        s3.set_high();
        couleur.blue += ((fn_read() as f32/freq_max as f32)*255.0) as u32;

        //On lis le rouge
        s2.set_low();
        s3.set_low();
        couleur.red += ((fn_read() as f32/freq_max as f32)*255.0) as u32;

        s0.set_low();
        s1.set_low();

    }

    couleur.blue /= nb_pass;
    couleur.green /= nb_pass;
    couleur.red /= nb_pass;

    couleur
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}