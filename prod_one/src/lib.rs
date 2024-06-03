pub struct ProdOneData {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use prod_two::ProdTwoData;
    use test_utils::Dummy;

    use crate::ProdOneData;

    #[test]
    fn prod_one_dummy() {
        let mut rng = rand::thread_rng();
        let Dummy::<ProdOneData>(data) = Faker.fake_with_rng(&mut rng);
    }

    #[test]
    fn prod_two_dummy() {
        let mut rng = rand::thread_rng();
        let Dummy::<ProdTwoData>(data) = Faker.fake_with_rng(&mut rng);
        assert_eq!(data.id.len(), 10);
    }
}
