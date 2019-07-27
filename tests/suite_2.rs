use hsm;

struct Context {
    first_entry: usize,
    second_entry: usize,
    third_entry: usize,
    first_action: usize,
    second_action: usize,
    third_action: usize,
    first_exit: usize,
    second_exit: usize,
    third_exit: usize,
}

enum Event {
    Initial,
    First,
    Second,
    Third,
    Up,
    Down,
}

type Transition = hsm::Transition<Context, Event>;

type StateMachine = hsm::StateMachine<Context, Event>;

struct RootState;
struct InitialState;
struct FirstState;
struct SecondState;
struct ThirdState;

impl hsm::State<Context, Event> for RootState {
    fn transition(&self, _context: &mut Context, _event: &Event) -> Transition {
        Transition::Internal(None)
    }
}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _context: &mut Context, _event: &Event) -> Transition {
        Transition::Local(&FIRST_STATE, None)
    }
}

impl FirstState {
    fn action(context: &mut Context, _event: &Event) {
        context.first_action += 1;
    }
}

impl hsm::State<Context, Event> for FirstState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.first_entry += 1;
    }

    fn transition(&self, _context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::First => Transition::Internal(Some(Self::action)),
            Event::Up => Transition::Internal(None),
            Event::Down => Transition::Local(&SECOND_STATE, None),
            _ => Transition::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.first_exit += 1;
    }
}

impl SecondState {
    fn action(context: &mut Context, _event: &Event) {
        context.second_action += 1;
    }
}

impl hsm::State<Context, Event> for SecondState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&FIRST_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.second_entry += 1;
    }

    fn transition(&self, _context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Second => Transition::Internal(Some(Self::action)),
            Event::Up => Transition::Local(&FIRST_STATE, None),
            Event::Down => Transition::Local(&THIRD_STATE, None),
            _ => Transition::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.second_exit += 1;
    }
}

impl ThirdState {
    fn action(context: &mut Context, _event: &Event) {
        context.third_action += 1;
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&SECOND_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.third_entry += 1;
    }

    fn transition(&self, _context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Third => Transition::Internal(Some(Self::action)),
            Event::Up => Transition::Local(&SECOND_STATE, None),
            Event::Down => Transition::Internal(None),
            _ => Transition::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.third_exit += 1;
    }
}

static ROOT_STATE: RootState = RootState;
static INITIAL_STATE: InitialState = InitialState;
static FIRST_STATE: FirstState = FirstState;
static SECOND_STATE: SecondState = SecondState;
static THIRD_STATE: ThirdState = ThirdState;

fn create_machine() -> StateMachine {
    StateMachine::new(&INITIAL_STATE)
}

fn initial_step(machine: &mut StateMachine, context: &mut Context) {
    let initial_event = Event::Initial;
    machine.dispatch(context, &initial_event);
}

#[test]
fn startup() {
    let mut context = Context {
        first_entry: 0,
        second_entry: 0,
        third_entry: 0,
        first_action: 0,
        second_action: 0,
        third_action: 0,
        first_exit: 0,
        second_exit: 0,
        third_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
}

fn first_step(machine: &mut StateMachine, context: &mut Context) {
    let first_event = Event::First;
    machine.dispatch(context, &first_event);
}

fn second_step(machine: &mut StateMachine, context: &mut Context) {
    let second_event = Event::Second;
    machine.dispatch(context, &second_event);
}

fn third_step(machine: &mut StateMachine, context: &mut Context) {
    let third_event = Event::Third;
    machine.dispatch(context, &third_event);
}

fn up_step(machine: &mut StateMachine, context: &mut Context) {
    let up_event = Event::Up;
    machine.dispatch(context, &up_event);
}

fn down_step(machine: &mut StateMachine, context: &mut Context) {
    let down_event = Event::Down;
    machine.dispatch(context, &down_event);
}

#[test]
fn multi_ladder() {
    let mut context = Context {
        first_entry: 0,
        second_entry: 0,
        third_entry: 0,
        first_action: 0,
        second_action: 0,
        third_action: 0,
        first_exit: 0,
        second_exit: 0,
        third_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
    assert_eq!(context.first_entry, 1);

    for i in 0..1000 {
        first_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.first_action, i + 1);

        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.second_entry, i + 1);

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.second_action, 2 * i + 1);

        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.third_entry, i + 1);

        third_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.third_action, i + 1);

        up_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.third_exit, i + 1);

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.second_action, 2 * i + 2);

        up_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.second_exit, i + 1);
    }
}
