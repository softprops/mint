use super::Makefile;

pub trait Lint {
    fn name(&self) -> String;
    fn apply(&self, makefile: &Makefile) -> Option<String>;
}

pub struct HasTarget {
    target: String
}

impl HasTarget {
    pub fn new(target: String) -> HasTarget {
        HasTarget { target: target }
    }
}

impl Lint for HasTarget {
    fn name(&self) -> String {
        "HasTarget".to_owned()
    }

    fn apply(&self, makefile: &Makefile) -> Option<String> {
        if let None = makefile.rules.iter().find(|r|r.target == self.target) {
            Some(format!("missing target {}", self.target))
        } else {
            None
        }
    }
}
