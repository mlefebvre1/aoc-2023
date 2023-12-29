use std::{cell::RefCell, cmp::Ordering, collections::HashMap, ops::Range, str::FromStr};

use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct Puzzle {
    workflows: Workflows,
    ratings: Ratings,
    combinations_total: RefCell<usize>,
}
impl Puzzle {
    pub fn run(&self) -> usize {
        let mut total = 0;
        for rating in self.ratings.0.iter() {
            let mut workflow_label = "in".to_string();
            loop {
                match self.workflows.0[&workflow_label].process(rating) {
                    Part::Accepted => {
                        total += rating.values().sum::<usize>();
                        break;
                    }
                    Part::Rejected => {
                        break;
                    }
                    Part::Workflow(new_label) => {
                        workflow_label = new_label;
                    }
                }
            }
        }
        total
    }

    pub fn explore(&self) -> usize {
        let ratings: HashMap<String, Range<usize>> = ["x", "m", "a", "s"]
            .map(|k| (k.to_string(), 1..4000))
            .into_iter()
            .collect();
        self.explore_workflow(&self.workflows.0["in"], ratings);
        *self.combinations_total.borrow()
    }

    fn explore_workflow(&self, wf: &Workflow, ratings: HashMap<String, Range<usize>>) {
        let mut ratings_ok = ratings.clone();
        let mut ratings_else = ratings.clone();
        if wf.cmp == Ordering::Less {
            ratings_ok.get_mut(&wf.left).unwrap().end = wf.right - 1;
            ratings_else.get_mut(&wf.left).unwrap().start = wf.right;
        } else {
            ratings_ok.get_mut(&wf.left).unwrap().start = wf.right + 1;
            ratings_else.get_mut(&wf.left).unwrap().end = wf.right;
        }

        self.explore_ans(&wf.ok, ratings_ok);
        self.explore_ans(&wf.orelse, ratings_else);
    }

    fn explore_ans(&self, ans: &Ans, ratings: HashMap<String, Range<usize>>) {
        match ans {
            Ans::Part(Part::Accepted) => {
                *self.combinations_total.borrow_mut() += ratings
                    .values()
                    .map(|r| (r.end - r.start) + 1)
                    .product::<usize>();
            }
            Ans::Part(Part::Rejected) => {}
            Ans::Part(Part::Workflow(label)) => {
                self.explore_workflow(&self.workflows.0[label], ratings);
            }
            Ans::Workflow(new_workflow) => {
                self.explore_workflow(new_workflow, ratings);
            }
        }
    }
}
impl FromStr for Puzzle {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let workflows_raw = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let workflows = Workflows::from_str(&workflows_raw)?;

        let ratings_raw = lines.collect::<Vec<_>>().join("\n");
        let ratings = Ratings::from_str(&ratings_raw)?;
        Ok(Self {
            workflows,
            ratings,
            combinations_total: RefCell::new(0),
        })
    }
}

#[derive(Debug)]
struct Workflows(HashMap<String, Workflow>);
impl FromStr for Workflows {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = HashMap::new();
        for line in s.lines() {
            let line_split = line.split_once('{').ok_or(anyhow!(""))?;
            let label = line_split.0.to_string();
            let workflow = Workflow::from_str(&line_split.1.replace('}', ""))?;
            inner.insert(label, workflow);
        }
        Ok(Self(inner))
    }
}

#[derive(Debug)]
struct Workflow {
    left: String,
    cmp: Ordering,
    right: usize,
    ok: Box<Ans>,
    orelse: Box<Ans>,
}
impl Workflow {
    fn process(&self, rating: &HashMap<String, usize>) -> Part {
        if rating[&self.left].cmp(&self.right) == self.cmp {
            Self::ans_process(&self.ok, rating)
        } else {
            Self::ans_process(&self.orelse, rating)
        }
    }
    fn ans_process(ans: &Ans, rating: &HashMap<String, usize>) -> Part {
        match ans {
            Ans::Part(part) => part.clone(),
            Ans::Workflow(new_workflow) => Self::process(new_workflow, rating),
        }
    }
}
impl FromStr for Workflow {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split_once(':').ok_or(anyhow!(""))?;
        let cond = ss.0;
        let cmp = if cond.contains('<') {
            Ordering::Less
        } else {
            Ordering::Greater
        };
        let mut cond_split = cond.split(['<', '>']);
        let left = cond_split.next().ok_or(anyhow!(""))?.to_string();
        let right = cond_split.next().ok_or(anyhow!(""))?.parse()?;
        let rest_split = ss.1.split_once(',').ok_or(anyhow!(""))?;
        let ok = Box::new(if let Ok(work) = Self::from_str(rest_split.0) {
            Ans::Workflow(work)
        } else {
            Ans::Part(Part::from_str(rest_split.0)?)
        });
        let orelse = Box::new(if let Ok(work) = Self::from_str(rest_split.1) {
            Ans::Workflow(work)
        } else {
            Ans::Part(Part::from_str(rest_split.1)?)
        });

        Ok(Self {
            left,
            cmp,
            right,
            ok,
            orelse,
        })
    }
}

#[derive(Debug)]
enum Variable {
    VarX,
    VarM,
    VarA,
    VarS,
}
impl FromStr for Variable {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Variable::VarX),
            "m" => Ok(Variable::VarM),
            "a" => Ok(Variable::VarA),
            "s" => Ok(Variable::VarS),
            _ => Err(anyhow!("failed to convert variable")),
        }
    }
}

#[derive(Debug, Clone)]
enum Part {
    Accepted,
    Rejected,
    Workflow(String),
}
impl FromStr for Part {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "A" {
            Ok(Self::Accepted)
        } else if s == "R" {
            Ok(Self::Rejected)
        } else {
            Ok(Self::Workflow(s.to_string()))
        }
    }
}

#[derive(Debug)]
enum Ans {
    Part(Part),
    Workflow(Workflow),
}

#[derive(Debug)]
struct Ratings(Vec<Rating>);
impl FromStr for Ratings {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s
            .lines()
            .map(|line| {
                let line = line.replace(['{', '}'], "");
                line.split(',')
                    .map(|r| {
                        let rr = r.split_once('=').unwrap();
                        let key = rr.0.to_string(); // x,m,a,s
                        let value = rr.1.parse().unwrap();
                        (key, value)
                    })
                    .collect()
            })
            .collect();

        Ok(Self(v))
    }
}
type Rating = HashMap<String, usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_workflow() {
        let s = "a<2006:qkq,m>2090:A,rfg";
        let workflow = Workflow::from_str(s).unwrap();
        println!("{workflow:?}");
    }

    #[test]
    fn test_parse_workflows() {
        let s = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}";
        let workflows = Workflows::from_str(s).unwrap();
        println!("{:?}", workflows.0["crn"]);
    }

    #[test]
    fn test_ratings() {
        let s = "{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}";
        let ratings = Ratings::from_str(s).unwrap();
        println!("{:?}", ratings);
    }
}
