#[derive(PartialEq, Debug)]
pub struct SvgPath {
    pub d: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct Svg {
    pub class: &'static str,
    pub xmlns: &'static str,
    pub view_box: &'static str,
    pub path: SvgPath,
}
