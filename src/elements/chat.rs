enum Kind {
    On,
    Off,
}

struct Child {
    pub state: Option<Kind>,
}

impl Child {
    fn update(&mut self) {
        self.state = Some(Kind.Off);
    }
}

struct Parent {
    child: Child,
}
