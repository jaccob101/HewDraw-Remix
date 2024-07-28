use super::*;

mod donkey;
mod ganon;
mod lucario;
mod pickel;
mod ptrainer;
mod littlemac;
mod reflet;
mod rockman;
mod krool;
mod brave;

mod ryu_shinkuhadoken;

mod weapon;

pub fn install() {
    donkey::install();
    ganon::install();
    lucario::install();
    pickel::install();
    ptrainer::install();
    littlemac::install();
    reflet::install();
    rockman::install();
    krool::install();
    brave::install();

    ryu_shinkuhadoken::install();

    weapon::install();
}