use tabled::Tabled;

#[derive(Tabled, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct RenderableMove<'a> {
    #[header("Name")]
    pub name: String,
    #[header("Type")]
    pub pk_type: String,
    #[header("Power")]
    pub power: i32,
    #[header("PP")]
    pub pp: i32,
    #[header("Accuracy")]
    pub accuracy: i32,
    #[header("Description")]
    pub description: String,
    #[header("Move Learn Method")]
    pub move_learn_method: &'a str,
    #[header("Level Learned At")]
    pub level_learned_at: i32,
}
