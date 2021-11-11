//! This is a parser crate for the [CookLang](https://github.com/cooklang/spec).  The main feature is parsing a String into a
//! struct that implements serde and can be easily used from there.
//!
//! The implementation is nearly fully complete. Only image tags are missing. They are just ignored by now.
//!

use super::ast::{Cookware, Ingredient, Metadata, RecipeLine, Step, StepItem, Timer};
use pest_consume::{match_nodes, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "../CookLang.pest"]
struct CookParser;

#[allow(dead_code)] // used in macro
type Pesult<T> = Result<T, pest_consume::Error<Rule>>;
#[allow(dead_code)] // used in macro
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl CookParser {
    fn EOI(_input: Node) -> Pesult<()> {
        Ok(())
    }

    fn recipe(input: Node) -> Pesult<Vec<RecipeLine>> {
        Ok(match_nodes!(input.into_children();
            [recipe_main(o), EOI(_)] => o,
        ))
    }

    fn recipe_main(input: Node) -> Pesult<Vec<RecipeLine>> {
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
            [step_content(i)] => StepItem::Content(i),
            [ingredient(i)] => StepItem::Ingredient(i),
            [cookware(i)] => StepItem::Cookware(i),
            [timer(i)] => StepItem::Timer(i),
        ))
    }

    fn step_content(input: Node) -> Pesult<&str> {
        Ok(input.as_str())
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

pub fn parse(input: &str) -> Pesult<Vec<RecipeLine>> {
    let nodes = CookParser::parse(Rule::recipe, input)?;
    CookParser::recipe(nodes.single()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input: &str) -> Vec<RecipeLine> {
        parse(input).unwrap()
    }

    fn test_multi(input: &str, output: Vec<RecipeLine>) {
        assert_eq!(test(input), output);
    }

    fn test_single(input: &str, output: RecipeLine) {
        test_multi(input, vec![output]);
    }

    fn md<'a>(key: &'a str, value: &'a str) -> RecipeLine<'a> {
        RecipeLine::Metadata(Metadata { key, value })
    }

    #[test]
    fn test_metadata() {
        test_single(">> servings: 2", md("servings", "2"));
        test_multi(
            "\t  >> a : b c \n \t >>3d:f g  ",
            vec![md("a", "b c"), md("3d", "f g")],
        );
    }

    #[test]
    fn basic_step() {
        test_single(
            " do something ",
            RecipeLine::Step(vec![StepItem::Content("do something")]),
        );
    }

    #[test]
    fn ingredient() {
        let r = RecipeLine::Step(vec![
            StepItem::Content("chop"),
            StepItem::Ingredient(Ingredient {
                name: "cucumber",
                amount: "",
            }),
            StepItem::Content("finely"),
        ]);
        test_single(" chop @cucumber finely", r.clone());
        test_single("chop@cucumber{}finely", r);
    }

    #[test]
    fn long_ingredient() {
        test_single(
            "sprinkle @ground pepper{} to taste",
            RecipeLine::Step(vec![
                StepItem::Content("sprinkle"),
                StepItem::Ingredient(Ingredient {
                    name: "ground pepper",
                    amount: "",
                }),
                StepItem::Content("to taste"),
            ]),
        );
    }

    #[test]
    fn ingredient_quantity() {
        test_single(
            "chop @red bell pepper{1kg}",
            RecipeLine::Step(vec![
                StepItem::Content("chop"),
                StepItem::Ingredient(Ingredient {
                    name: "red bell pepper",
                    amount: "1kg",
                }),
            ]),
        );
    }

    #[test]
    fn cookware() {
        test_single(
            "#knife",
            RecipeLine::Step(vec![StepItem::Cookware(Cookware { name: "knife" })]),
        );
    }

    #[test]
    fn long_cookware() {
        test_single(
            "chop @cheese with #long knife{}",
            RecipeLine::Step(vec![
                StepItem::Content("chop"),
                StepItem::Ingredient(Ingredient {
                    name: "cheese",
                    amount: "",
                }),
                StepItem::Content("with"),
                StepItem::Cookware(Cookware { name: "long knife" }),
            ]),
        );
    }

    #[test]
    fn timer() {
        test_single(
            "cook @eggs{2} in #skillet for ~{25%minutes}.",
            RecipeLine::Step(vec![
                StepItem::Content("cook"),
                StepItem::Ingredient(Ingredient {
                    name: "eggs",
                    amount: "2",
                }),
                StepItem::Content("in"),
                StepItem::Cookware(Cookware { name: "skillet" }),
                StepItem::Content("for"),
                StepItem::Timer(Timer {
                    duration: 25,
                    unit: "minutes",
                }),
                StepItem::Content("."),
            ]),
        );
    }

    #[test]
    fn blank_lines() {
        test_multi(
            "       a       \n    \n\nb\n\n",
            vec![
                RecipeLine::Step(vec![StepItem::Content("a")]),
                RecipeLine::Step(vec![StepItem::Content("b")]),
            ],
        );
    }
}
