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

fn initialize(context: &mut Context) -> hsm::StateMachine<Context, Event> {
    let mut machine = hsm::StateMachine::<Context, Event>::new(&INITIAL_STATE);
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    let initial_event = Event::Initial;
    machine.dispatch(context, &initial_event);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));

    machine
}

#[test]
fn startup() {
    let mut context = Context;
    initialize(&mut context);
}

fn first_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let first_event = Event::First;
    machine.dispatch(context, &first_event);
    assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
}

fn second_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let second_event = Event::Second;
    machine.dispatch(context, &second_event);
    assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
}

fn third_step(machine: &mut hsm::StateMachine<Context, Event>, context: &mut Context) {
    let third_event = Event::Third;
    machine.dispatch(context, &third_event);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
}

#[test]
fn multi_loop() {
    let mut context = Context;
    let mut machine = initialize(&mut context);

    for _ in 0..1000 {
        first_step(&mut machine, &mut context);
        second_step(&mut machine, &mut context);
        third_step(&mut machine, &mut context);
    }
}
