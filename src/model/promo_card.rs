use super::img::Image;

#[derive(PartialEq, Debug)]
pub struct PromoCard {
    pub promo_type: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub img: Image,
}
