#![no_std]

const MAX_DEPTH: usize = 8;

pub enum Transition<C: 'static, E: 'static> {
    Unknown,
    Internal,
    Local(&'static dyn State<C, E>),
    External(&'static dyn State<C, E>),
}

pub trait State<C: 'static, E: 'static> {
    fn parent(&self) -> Option<&'static dyn State<C, E>> {
        None
    }

    fn entry(&self, _context: &mut C) {
    }

    fn process(&self, _context: &mut C, _event: &E) -> Transition<C, E> {
        Transition::<C, E>::Unknown
    }

    fn exit(&self, _context: &mut C) {
    }
}

pub struct StateMachine<C: 'static, E: 'static> {
    root_state: &'static dyn State<C, E>,
    active_state: &'static dyn State<C, E>,
}

impl<C: 'static, E: 'static> StateMachine<C, E> {
    pub fn new(initial_state: &'static dyn State<C, E>) -> Self {
        let mut topmost_state = initial_state;

        while let Some(parent_state) = topmost_state.parent() {
            topmost_state = parent_state;
        }

        Self {
            root_state: topmost_state,
            active_state: initial_state,
        }
    }

    pub fn active(&self) -> &'static dyn State<C, E> {
        self.active_state
    }

    pub fn dispatch(&mut self, context: &mut C, event: &E) {
        match self.process(context, event) {
            Transition::<C, E>::External(target_state) => {
                self.traverse(context, target_state, false);
            },
            Transition::<C, E>::Local(target_state) => {
                self.traverse(context, target_state, true);
            },
            Transition::<C, E>::Internal => {
                self.traverse(context, self.active_state, true);
            },
            Transition::<C, E>::Unknown => {
                panic!("Unhandled event passed through root state!");
            },
        }
    }

    fn process(&self, context: &mut C, event: &E) -> Transition<C, E> {
        let mut transition = Transition::<C, E>::Unknown;
        let mut effective_state = self.active_state;

        while let Transition::<C, E>::Unknown = transition {
            transition = effective_state.process(context, event);

            match effective_state.parent() {
                Some(parent_state) => effective_state = parent_state,
                None => break,
            }
        }

        transition
    }

    fn traverse(&mut self, context: &mut C, target_state: &'static dyn State<C, E>, local: bool) {
        let mut sources: [&'static dyn State<C, E>; MAX_DEPTH] = [self.root_state; MAX_DEPTH];
        let mut targets: [&'static dyn State<C, E>; MAX_DEPTH] = [self.root_state; MAX_DEPTH];
        let mut source_depth = 1;
        let mut target_depth = 1;
        let mut source_top = 0;
        let mut target_top = 0;

        sources[0] = self.active_state;
        targets[0] = target_state;

        if !core::ptr::eq(sources[0], targets[0]) {
            let mut topmost_state;

            topmost_state = sources[0];

            while let Some(parent_state) = topmost_state.parent() {
                if source_depth == MAX_DEPTH {
                    panic!("State tree depth limit exceeded!");
                }

                topmost_state = parent_state;

                sources[source_depth] = topmost_state;
                source_depth += 1;
            }

            topmost_state = targets[0];

            while let Some(parent_state) = topmost_state.parent() {
                if target_depth == MAX_DEPTH {
                    panic!("State tree depth limit exceeded!");
                }

                topmost_state = parent_state;

                targets[target_depth] = topmost_state;
                target_depth += 1;
            }

            let mut common_ancestor = None;

            'outer: for i in 0..source_depth {
                'inner: for j in 0..target_depth {
                    if core::ptr::eq(sources[i], targets[j]) {
                        common_ancestor = Some(sources[i]);
                        source_top = i;
                        target_top = j;
                        break 'outer;
                    }
                }
            }

            common_ancestor.expect("Common ancestor has not been found!");
        }

        for i in 0..source_top {
            sources[i].exit(context);
        }

        if !local {
            sources[source_top].exit(context);
            targets[target_top].entry(context);
        }

        for j in (0..target_top).rev() {
            targets[j].entry(context);
        }

        self.active_state = target_state;
    }
}
