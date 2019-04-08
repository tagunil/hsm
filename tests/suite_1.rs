use hsm;

struct Context {
    first: usize,
    second: usize,
    third: usize,
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

impl hsm::State<Context, Event> for RootState {
}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _: &mut Context, _: &Event) -> Transition {
        hsm::Transition::<Context, Event>::Local(&FIRST_STATE, None)
    }
}

impl hsm::State<Context, Event> for FirstState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::First => {
                context.first += 1;
                hsm::Transition::<Context, Event>::Local(&SECOND_STATE, None)
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl hsm::State<Context, Event> for SecondState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Second => {
                context.second += 1;
                hsm::Transition::<Context, Event>::Local(&THIRD_STATE, None)
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Third => {
                context.third += 1;
                hsm::Transition::<Context, Event>::Local(&FIRST_STATE, None)
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
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
        first: 0,
        second: 0,
        third: 0,
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
        first: 0,
        second: 0,
        third: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));

    for i in 0..1000 {
        first_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.first, i + 1);

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.second, i + 1);

        third_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.third, i + 1);
    }
}

#[test]
#[should_panic(expected = "Unhandled event passed through root state!")]
fn unhandled_event() {
    let mut context = Context {
        first: 0,
        second: 0,
        third: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));

    first_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

    first_step(&mut machine, &mut context);
}
