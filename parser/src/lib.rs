//! This is a parser crate for the [CookLang](https://github.com/cooklang/spec).  The main feature is parsing a String into a
//! struct that implements serde and can be easily used from there.
//!
//! The implementation is nearly fully complete. Only image tags are missing. They are just ignored by now.
//!

use pest_consume::match_nodes;

#[derive(pest_derive::Parser)]
#[grammar = "../CookLang.pest"]
struct CookParser;

#[allow(dead_code)] // used in macro
type Pesult<T> = Result<T, pest_consume::Error<Rule>>;
#[allow(dead_code)] // used in macro
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

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

type Step<'a> = Vec<StepItem<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub enum StepItem<'a> {
    Content(String),
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

#[pest_consume::parser]
impl CookParser {
    fn recipe(input: Node) -> Pesult<Vec<RecipeLine>> {
        Ok(match_nodes!(input.into_children();
            [line(l)..] => l.filter_map(|x| x).collect(),
        ))
    }

    fn line(input: Node) -> Pesult<Option<RecipeLine>> {
        Ok(match_nodes!(input.into_children();
            [metadata_item(m)] => Some(RecipeLine::Metadata(m)),
            [step(s)] => Some(RecipeLine::Step(s)),
            [_] => None
        ))
    }

    fn metadata_item(input: Node) -> Pesult<Metadata> {
        Ok(match_nodes!(input.into_children();
            [metadata_key(k), metadata_value(v)] => Metadata { key: k, value: v },
        ))
    }

    fn metadata_key(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn metadata_value(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn step(input: Node) -> Pesult<Vec<StepItem>> {
        Ok(match_nodes!(input.into_children();
            [step_part(p)..] => p.collect()
        ))
    }

    fn step_part(input: Node) -> Pesult<StepItem> {
        Ok(match_nodes!(input.into_children();
            [ingredient(i)] => StepItem::Ingredient(i),
            [cookware(i)] => StepItem::Cookware(i),
            [timer(i)] => StepItem::Timer(i),
        ))
    }

    fn ingredient(input: Node) -> Pesult<Ingredient> {
        Ok(match_nodes!(input.into_children();
            [short_ingredient(i)] => i,
            [long_ingredient(i)] => i,
        ))
    }

    fn short_ingredient(input: Node) -> Pesult<Ingredient> {
        Ok(Ingredient {
            name: input.as_str(),
            amount: "",
        })
    }

    fn long_ingredient(input: Node) -> Pesult<Ingredient> {
        Ok(match_nodes!(input.into_children();
            [inline_name(n), quantity(q)] => Ingredient {
                name: n,
                amount: q,
            }
        ))
    }

    fn inline_name(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn quantity(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }

    fn cookware(input: Node) -> Pesult<Cookware> {
        Ok(match_nodes!(input.into_children();
            [short_cookware(i)] => i,
            [long_cookware(i)] => i,
        ))
    }

    fn short_cookware(input: Node) -> Pesult<Cookware> {
        Ok(Cookware {
            name: input.as_str(),
        })
    }

    fn long_cookware(input: Node) -> Pesult<Cookware> {
        Ok(match_nodes!(input.into_children();
            [inline_name(n)] => Cookware {
                name: n,
            }
        ))
    }

    fn timer(input: Node) -> Pesult<Timer> {
        Ok(match_nodes!(input.into_children();
            [timer_duration(d), timer_unit(u)] => Timer {
                duration: d,
                unit: u
            }
        ))
    }

    fn timer_duration(input: Node) -> Pesult<i64> {
        Ok(input.as_str().parse().unwrap())
    }

    fn timer_unit(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
    }
}

fn parse(input: &str) -> Pesult<Vec<RecipeLine>> {
    CookParser::parse(Rule::recipe, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str) {}

    fn test_metadata() {}
}
