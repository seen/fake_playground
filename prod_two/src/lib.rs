pub struct ProdTwoData {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use prod_one::ProdOneData;
    use test_utils::Dummy;

    #[test]
    fn prod_one_dummy() {
        let mut rng = rand::thread_rng();
        let Dummy::<ProdOneData>(prod_one) = Faker.fake_with_rng(&mut rng);
        assert_eq!(prod_one.id.len(), 20);
    }
}
