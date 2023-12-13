use super::svg::Svg;

#[derive(PartialEq, Debug)]
pub struct HomeCardIcon {
    pub svg: Svg,
    pub title: &'static str,
}
