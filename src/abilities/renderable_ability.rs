use tabled::Tabled;

#[derive(Tabled, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct RenderableAbility {
    #[header("Name")]
    pub name: String,
    #[header("Short Effect")]
    pub short_effect: String,
    #[header("Effect")]
    pub effect: String,
    #[header("Description")]
    pub description: String,
    #[header("Is Hidden")]
    pub is_hidden: bool,
}
