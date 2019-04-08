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
    fn transition(&self, _: &mut Context, _: &Event) -> Transition {
        hsm::Transition::<Context, Event>::Internal(None)
    }
}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _: &mut Context, _: &Event) -> Transition {
        hsm::Transition::<Context, Event>::Local(&FIRST_STATE, None)
    }
}

impl FirstState {
    fn action(context: &mut Context, _: &Event) {
        context.first += 1;
    }
}

impl hsm::State<Context, Event> for FirstState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::First => hsm::Transition::<Context, Event>::Internal(Some(Self::action)),
            Event::Up => hsm::Transition::<Context, Event>::Internal(None),
            Event::Down => hsm::Transition::<Context, Event>::Local(&SECOND_STATE, None),
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl SecondState {
    fn action(context: &mut Context, _: &Event) {
        context.second += 1;
    }
}

impl hsm::State<Context, Event> for SecondState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&FIRST_STATE)
    }

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Second => hsm::Transition::<Context, Event>::Internal(Some(Self::action)),
            Event::Up => hsm::Transition::<Context, Event>::Local(&FIRST_STATE, None),
            Event::Down => hsm::Transition::<Context, Event>::Local(&THIRD_STATE, None),
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }
}

impl ThirdState {
    fn action(context: &mut Context, _: &Event) {
        context.third += 1;
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&SECOND_STATE)
    }

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Third => hsm::Transition::<Context, Event>::Internal(Some(Self::action)),
            Event::Up => hsm::Transition::<Context, Event>::Local(&SECOND_STATE, None),
            Event::Down => hsm::Transition::<Context, Event>::Internal(None),
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
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.first, i + 1);

        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.second, 2 * i + 1);

        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));

        third_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.third, i + 1);

        up_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));

        second_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.second, 2 * i + 2);

        up_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
    }
}
