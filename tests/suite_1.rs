use hsm;

struct Context;

enum Event {
    Initial,
    First,
    Second,
    Third,
}

struct RootState;
struct InitialState;
struct FirstState;
struct SecondState;
struct ThirdState;

impl hsm::State<Context, Event> for RootState {
}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn process(&self, _context: &mut Context, _event: &Event) -> hsm::Transition<Context, Event> {
        hsm::Transition::<Context, Event>::Local(&FIRST_STATE)
    }
}

impl hsm::State<Context, Event> for FirstState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn process(&self, _context: &mut Context, event: &Event) -> hsm::Transition<Context, Event> {
        match event {
            Event::First => hsm::Transition::<Context, Event>::Local(&SECOND_STATE),
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl hsm::State<Context, Event> for SecondState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn process(&self, _context: &mut Context, event: &Event) -> hsm::Transition<Context, Event> {
        match event {
            Event::Second => hsm::Transition::<Context, Event>::Local(&THIRD_STATE),
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn process(&self, _context: &mut Context, event: &Event) -> hsm::Transition<Context, Event> {
        match event {
            Event::Third => hsm::Transition::<Context, Event>::Local(&FIRST_STATE),
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

static ROOT_STATE: RootState = RootState;
static INITIAL_STATE: InitialState = InitialState;
static FIRST_STATE: FirstState = FirstState;
static SECOND_STATE: SecondState = SecondState;
static THIRD_STATE: ThirdState = ThirdState;

fn create_machine() -> hsm::StateMachine<Context, Event> {
    hsm::StateMachine::<Context, Event>::new(&INITIAL_STATE)
}

fn initial_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let initial_event = Event::Initial;
    machine.dispatch(context, &initial_event);
}

#[test]
fn startup() {
    let mut context = Context;
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
}

fn first_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let first_event = Event::First;
    machine.dispatch(context, &first_event);
}

fn second_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let second_event = Event::Second;
    machine.dispatch(context, &second_event);
}

fn third_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let third_event = Event::Third;
    machine.dispatch(context, &third_event);
}

#[test]
fn multi_loop() {
    let mut context = Context;
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));

    for _ in 0..1000 {
        first_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));

        third_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
    }
}

#[test]
#[should_panic(expected = "Unhandled event passed through root state!")]
fn unhandled_event() {
    let mut context = Context;
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));

    first_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

    first_step(&mut machine, &mut context);
}
