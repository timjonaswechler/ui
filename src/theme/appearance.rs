#[derive(Copy, Clone, Debug)]
pub enum Appearance {
    Light,
    Dark,
}
impl Default for Appearance {
    fn default() -> Self {
        Appearance::Light
    }
}
impl Appearance {
    pub fn is_light(&self) -> bool {
        matches!(self, Appearance::Light)
    }
    pub fn is_dark(&self) -> bool {
        matches!(self, Appearance::Dark)
    }

    pub fn toggle(&mut self) {
        *self = match self {
            Appearance::Light => Appearance::Dark,
            Appearance::Dark => Appearance::Light,
        };
    }

    pub fn to_string(&self) -> &str {
        match self {
            Appearance::Light => "light",
            Appearance::Dark => "dark",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "light" => Appearance::Light,
            "dark" => Appearance::Dark,
            _ => Appearance::default(),
        }
    }
    pub fn get(&self) -> &'static str {
        match self {
            Appearance::Light => "light",
            Appearance::Dark => "dark",
        }
    }
    pub fn set(&mut self, s: &str) {
        *self = match s {
            "light" => Appearance::Light,
            "dark" => Appearance::Dark,
            _ => *self,
        };
    }
}
