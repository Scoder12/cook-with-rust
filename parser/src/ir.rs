#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    /// Raw source code of the recipe that this struct has been generated from.
    pub source: String,
    /// Contains the metadata of the recipe. Provided in the form of [Metadata].
    pub metadata: Metadata,
    /// Contains reduced instructions.
    ///
    /// For every mentioning of a ingredient there is an @ in replacement. The mentioning directly
    /// links to an [IngredientSpecifier].
    ///
    /// For every mentioning of a cookware there is an # in replacement. The mentioning directly
    /// links to a [String] describing the cookware.
    ///
    /// For every mentioning of a timer there is an ~ in replacement. The mentioning directly links
    /// to a [Timer].
    pub instruction: String,
}

/// The metadata from the recipe is described in this metadata struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    /// Amount of servings. Is optional.
    pub servings: Option<Vec<usize>>,
    /// Other optional metadata contained in a [HashMap].
    pub ominous: HashMap<String, String>,
    /// Exact description of an [Ingredient] indexed by name.
    pub ingredients: HashMap<String, Ingredient>,
    /// Ingredient Specifier describing the mentioning of a [Ingredient]. The n-th mention of @
    /// in [Recipe::instruction] is the n-th [IngredientSpecifier] in this [Vec].
    pub ingredients_specifiers: Vec<IngredientSpecifier>,
    /// The n-th mention of # in [Recipe::instruction] is the n-th [String] in this [Vec].
    pub cookware: Vec<String>,
    /// The n-th mention of ~ in [Recipe::instruction] is the n-th [Timer] in this [Vec].
    pub timer: Vec<Timer>,
}

impl Metadata {
    fn add_key_value(&mut self, key: String, value: String) {
        self.ominous.insert(key, value);
    }
}
/// A Timer.
///
/// Describing the timer you have to set in this mentioning in the instructions.
#[derive(Debug, Serialize, Deserialize)]
pub struct Timer {
    /// The number of [Timer::unit]s in this Timer mentioning.
    pub amount: f64,
    /// The unit of this Timer contained in a [String].
    pub unit: String,
}

/// IngredientSpecifier
///
/// References to a [Ingredient] in [Metadata::ingredients] by [String].
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngredientSpecifier {
    /// Name of the ingredient this specifier references to. Have to be extracted from [Metadata::ingredients].
    pub ingredient: String,
    /// [Amount] to be used in this step.
    pub amount_in_step: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    /// Name of the ingredient.
    pub name: String,
    /// Uuid is currently not used.
    pub id: Uuid,
    /// Optional [Amount] specifier.
    pub amount: Option<Amount>,
    /// Unit this ingredient is measured in.
    pub unit: Option<String>,
}

/// Specifies the amount of a [Ingredient].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Amount {
    /// Scalable amount.
    ///
    /// To get the needed amount in the step or total needed amount [Amount::Multi::0] has to be
    /// multiplied by the servings.
    Multi(f64),
    /// Static Servings amount.
    Servings(Vec<f64>),
    /// Static amount.
    Single(f64),
}

impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Amount::Multi(a) => match rhs {
                Amount::Multi(b) => Amount::Multi(a + b),
                _ => {
                    panic!("Unallowed Addition");
                }
            },
            Amount::Servings(a) => match rhs {
                Amount::Servings(b) => {
                    Amount::Servings(a.iter().zip(b.iter()).map(|e| *e.0 + *e.1).collect())
                }
                _ => {
                    panic!("Unallowed Addition");
                }
            },
            Amount::Single(a) => match rhs {
                Amount::Single(b) => Amount::Single(a + b),
                _ => {
                    panic!("Unallowed Addition");
                }
            },
        }
    }
}