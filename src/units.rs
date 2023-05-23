use typenum::{N1, N2, P1, P2, Z0};
use uom::si::{self, f32::AmountOfSubstance, Quantity};

unit! {
    system: si;
    quantity: si::amount_of_substance;

    @euro: prefix!(none); "€", "euro", "euros";
    @cent: prefix!(centi); "c", "cent", "cent";
}

pub type ElectricityRate =
    Quantity<uom::si::ISQ<N2, N1, P2, Z0, Z0, P1, Z0>, uom::si::SI<f64>, f64>;

pub type Currency = AmountOfSubstance;
