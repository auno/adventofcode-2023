use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use strum_macros::EnumString;
use anyhow::{bail, Context, Error, Result};

#[derive(Copy, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone)]
enum Target {
    Accept,
    Reject,
    Redirect(String),
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if !(s == "A" || s == "R" || s.chars().all(|c| c.is_ascii_lowercase())) {
            bail!("Invalid target: {s}");
        }

        Ok(
            match s {
                "A" => Target::Accept,
                "R" => Target::Reject,
                _ => Target::Redirect(s.to_string()),
            }
        )
    }
}

#[derive(Clone)]
enum Rule {
    GreaterThan(Category, usize, Target),
    LessThan(Category, usize, Target),
    Unconditional(Target),
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(target) = s.parse() {
            return Ok(Rule::Unconditional(target));
        }

        let category = s[0..1].parse()?;
        let operator = &s[1..2];
        let colon_pos = s.chars().position(|c| c == ':').context(format!("Invalid rule; no colon: {s}"))?;
        let value = s[2..colon_pos].parse()?;
        let target = s[(colon_pos + 1)..].parse()?;

        Ok(
           match operator {
               ">" => Rule::GreaterThan(category, value, target),
               "<" => Rule::LessThan(category, value, target),
               _ => bail!("Invalid rule; unknown operator: {s}"),
           }

        )
    }
}

type Workflow = Vec<Rule>;

#[derive(Copy, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(r: &str) -> Result<Self> {
        let (mut x, mut m, mut a, mut s) = (None, None, None, None);

        for rating in (r[1..(r.len() - 1)]).split(',') {
            let category = rating[0..1].parse()?;
            let value = rating[2..].parse()?;

            match category {
                Category::X => { x = Some(value); },
                Category::M => { m = Some(value); },
                Category::A => { a = Some(value); },
                Category::S => { s = Some(value); },
            }
        }

        Ok(Part {
            x: x.context(format!("Invalid part; x rating missing: {r}"))?,
            m: m.context(format!("Invalid part; m rating missing: {r}"))?,
            a: a.context(format!("Invalid part; a rating missing: {r}"))?,
            s: s.context(format!("Invalid part; s rating missing: {r}"))?,
        })

    }
}

impl Part {
    fn rating(&self, category: Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

type Input = (HashMap<String, Workflow>, Vec<Part>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Result<Input> {
    let (workflows, parts) = input.split_once("\n\n").context("Invalid input")?;

    let workflows = workflows
        .lines()
        .map(|workflow| {
            let curly_pos = workflow.chars().position(|c| c == '{').context(format!("Invalid workflow; no curly bracket: {workflow}"))?;
            let name = workflow[0..curly_pos].to_string();
            let rules = workflow[(curly_pos + 1)..(workflow.len() - 1)]
                .split(',')
                .map(str::parse)
                .collect::<Result<_>>()?;

            Ok((name, rules))
        })
        .collect::<Result<_>>()?;

    let parts = parts
        .lines()
        .map(str::parse)
        .collect::<Result<_>>()?;

    Ok((workflows, parts))
}

fn evaluate(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut workflow = workflows.get("in").unwrap();

    loop {
        for rule in workflow {
            let target = match rule {
                Rule::GreaterThan(c, v, t) => {
                    if part.rating(*c) <= *v {
                        continue;
                    }

                    t
                },
                Rule::LessThan(c, v, t) => {
                    if part.rating(*c) >= * v {
                        continue;
                    }

                    t
                },
                Rule::Unconditional(t) => t,
            };

            match target {
                Target::Accept => { return true; }
                Target::Reject => { return false; }
                Target::Redirect(workflow_name) => {
                    workflow = workflows.get(workflow_name).unwrap();
                    break;
                }
            }
        }
    }
}

#[aoc(day19, part1)]
fn part1((workflows, parts): &Input) -> usize {
    parts
        .iter()
        .filter(|part| evaluate(workflows, part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

#[derive(Copy, Clone)]
struct Bounds {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

fn set_lower_bounds(mut bounds: Bounds, category: Category, value: usize) -> Bounds {
    match category {
        Category::X => { bounds.x.0 = value; }
        Category::M => { bounds.m.0 = value; }
        Category::A => { bounds.a.0 = value; }
        Category::S => { bounds.s.0 = value; }
    }

    bounds
}

fn set_upper_bounds(mut bounds: Bounds, category: Category, value: usize) -> Bounds {
    match category {
        Category::X => { bounds.x.1 = value; }
        Category::M => { bounds.m.1 = value; }
        Category::A => { bounds.a.1 = value; }
        Category::S => { bounds.s.1 = value; }
    }

    bounds
}

fn target_combinations(workflows: &HashMap<String, Workflow>, target: &Target, bounds: Bounds) -> usize {
    match target {
        Target::Accept => (bounds.x.1 - bounds.x.0) * (bounds.m.1 - bounds.m.0) * (bounds.a.1 - bounds.a.0) * (bounds.s.1 - bounds.s.0),
        Target::Reject => 0,
        Target::Redirect(workflow_name) => workflow_combinations(workflows, workflows.get(workflow_name).unwrap(), bounds),
    }
}

fn workflow_combinations(workflows: &HashMap<String, Workflow>, workflow: &Workflow, mut bounds: Bounds) -> usize {
    let mut num_combinations = 0;

    for rule in workflow {
        match rule {
            Rule::GreaterThan(c, v, t) => {
                num_combinations += target_combinations(workflows, t, set_lower_bounds(bounds, *c, *v + 1));
                bounds = set_upper_bounds(bounds, *c, *v + 1);
            },
            Rule::LessThan(c, v, t) => {
                num_combinations += target_combinations(workflows, t, set_upper_bounds(bounds, *c, *v));
                bounds = set_lower_bounds(bounds, *c, *v);
            },
            Rule::Unconditional(t) => {
                num_combinations += target_combinations(workflows, t, bounds);
                break;
            },
        }
    }

    num_combinations
}

#[aoc(day19, part2)]
fn part2((workflows, _): &Input) -> usize {
    let bounds = Bounds{
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };

    workflow_combinations(workflows, workflows.get("in").unwrap(), bounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(19114, part1(&parse(include_str!("../test_input/day19.part1.19114.txt")).unwrap()));
    }

    #[test]
    fn part1_input() {
        assert_eq!(399284, part1(&parse(include_str!("../input/2023/day19.txt")).unwrap()));
    }

    #[test]
    fn part2_example1() {
        assert_eq!(167409079868000, part2(&parse(include_str!("../test_input/day19.part2.167409079868000.txt")).unwrap()));
    }

    #[test]
    fn part2_input() {
        assert_eq!(121964982771486, part2(&parse(include_str!("../input/2023/day19.txt")).unwrap()));
    }
}
