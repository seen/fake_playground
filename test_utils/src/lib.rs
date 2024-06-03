use fake::{Fake, Faker};
use prod_one::ProdOneData;
use prod_two::ProdTwoData;
use rand::Rng;

pub struct Dummy<U>(pub U);

impl fake::Dummy<Faker> for Dummy<ProdOneData> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id: String = (20).fake_with_rng(rng);
        Dummy(ProdOneData { id })
    }
}

impl fake::Dummy<Faker> for Dummy<ProdTwoData> {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let id: String = (10).fake_with_rng(rng);
        Dummy(ProdTwoData { id })
    }
}
