use crate::card::Card;

pub trait Player {
    fn draw(&mut self, cards: Vec<Card>);

    fn exchange<F>(&mut self, exchange: F)
    where
        F: FnOnce(Vec<Card>) -> Vec<Card>;

    fn cards(&mut self) -> Vec<Card>;
}
