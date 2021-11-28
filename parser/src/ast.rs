type Step<'a> = Vec<StepItem<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub enum RecipeLine<'a> {
    Metadata(Metadata<'a>),
    Step(Step<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StepItem<'a> {
    Content(&'a str),
    Ingredient(Ingredient<'a>),
    Cookware(Cookware<'a>),
    Timer(Timer<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ingredient<'a> {
    pub name: &'a str,
    pub amount: &'a str, // TODO: Parse this
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookware<'a> {
    pub name: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Timer<'a> {
    pub duration: i64,
    pub unit: &'a str,
}
