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
}

type Transition = hsm::Transition<Context, Event>;

type StateMachine = hsm::StateMachine<Context, Event>;

struct RootState;
struct InitialState;
struct FirstState;
struct SecondState;
struct ThirdState;

impl hsm::State<Context, Event> for RootState {}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _context: &mut Context, _event: &Event) -> Transition {
        Transition::Local(&FIRST_STATE, None)
    }
}

impl hsm::State<Context, Event> for FirstState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.first_entry += 1;
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::First => {
                context.first_action += 1;
                Transition::Local(&SECOND_STATE, None)
            }
            _ => Transition::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.first_exit += 1;
    }
}

impl hsm::State<Context, Event> for SecondState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.second_entry += 1;
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Second => {
                context.second_action += 1;
                Transition::Local(&THIRD_STATE, None)
            }
            _ => Transition::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.second_exit += 1;
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.third_entry += 1;
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Third => {
                context.third_action += 1;
                Transition::Local(&FIRST_STATE, None)
            }
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

#[test]
fn multi_loop() {
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
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.first_exit, i + 1);
        assert_eq!(context.first_action, i + 1);
        assert_eq!(context.second_entry, i + 1);

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.second_exit, i + 1);
        assert_eq!(context.second_action, i + 1);
        assert_eq!(context.third_entry, i + 1);

        third_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.third_exit, i + 1);
        assert_eq!(context.third_action, i + 1);
        assert_eq!(context.first_entry, i + 2);
    }
}

#[test]
#[should_panic(expected = "Unhandled event passed through root state!")]
fn unhandled_event() {
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

    first_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

    first_step(&mut machine, &mut context);
}
