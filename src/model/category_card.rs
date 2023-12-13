use super::img::Image;

#[derive(PartialEq, Debug)]
pub struct CategoryCard {
    pub title: &'static str,
    pub text: &'static str,
    pub img: Image,
    pub bg_color: &'static str,
}
