use hsm;

struct Context {
    first_entry: usize,
    second_entry: usize,
    third_entry: usize,
    fourth_entry: usize,
    first_action: usize,
    second_action: usize,
    third_action: usize,
    fourth_action: usize,
    first_exit: usize,
    second_exit: usize,
    third_exit: usize,
    fourth_exit: usize,
}

enum Event {
    Initial,
    Jump,
    Down,
}

type Transition = hsm::Transition<Context, Event>;

type StateMachine = hsm::StateMachine<Context, Event>;

struct RootState;
struct InitialState;
struct FirstState;
struct SecondState;
struct ThirdState;
struct FourthState;

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

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Down => {
                hsm::Transition::<Context, Event>::Local(&SECOND_STATE, Some(Self::action))
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.first_exit += 1;
    }
}

impl SecondState {
    fn action(context: &mut Context, _: &Event) {
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

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Jump => {
                hsm::Transition::<Context, Event>::Local(&THIRD_STATE, Some(Self::action))
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.second_exit += 1;
    }
}

impl ThirdState {
    fn action(context: &mut Context, _: &Event) {
        context.third_action += 1;
    }
}

impl hsm::State<Context, Event> for ThirdState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.third_entry += 1;
    }

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Down => {
                hsm::Transition::<Context, Event>::Local(&FOURTH_STATE, Some(Self::action))
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.third_exit += 1;
    }
}

impl FourthState {
    fn action(context: &mut Context, _: &Event) {
        context.fourth_action += 1;
    }
}

impl hsm::State<Context, Event> for FourthState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&THIRD_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.fourth_entry += 1;
    }

    fn transition(&self, _: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Jump => {
                hsm::Transition::<Context, Event>::Local(&FIRST_STATE, Some(Self::action))
            },
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.fourth_exit += 1;
    }
}

static ROOT_STATE: RootState = RootState;
static INITIAL_STATE: InitialState = InitialState;
static FIRST_STATE: FirstState = FirstState;
static SECOND_STATE: SecondState = SecondState;
static THIRD_STATE: ThirdState = ThirdState;
static FOURTH_STATE: FourthState = FourthState;

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
        fourth_entry: 0,
        first_action: 0,
        second_action: 0,
        third_action: 0,
        fourth_action: 0,
        first_exit: 0,
        second_exit: 0,
        third_exit: 0,
        fourth_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
}

fn jump_step(machine: &mut StateMachine, context: &mut Context) {
    let jump_event = Event::Jump;
    machine.dispatch(context, &jump_event);
}

fn down_step(machine: &mut StateMachine, context: &mut Context) {
    let down_event = Event::Down;
    machine.dispatch(context, &down_event);
}

#[test]
fn multi_jump() {
    let mut context = Context {
        first_entry: 0,
        second_entry: 0,
        third_entry: 0,
        fourth_entry: 0,
        first_action: 0,
        second_action: 0,
        third_action: 0,
        fourth_action: 0,
        first_exit: 0,
        second_exit: 0,
        third_exit: 0,
        fourth_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
    assert_eq!(context.first_entry, 1);

    for i in 0..1000 {
        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &SECOND_STATE));
        assert_eq!(context.first_action, i + 1);
        assert_eq!(context.second_entry, i + 1);

        jump_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THIRD_STATE));
        assert_eq!(context.second_exit, i + 1);
        assert_eq!(context.first_exit, i + 1);
        assert_eq!(context.second_action, i + 1);
        assert_eq!(context.third_entry, i + 1);

        down_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FOURTH_STATE));
        assert_eq!(context.third_action, i + 1);
        assert_eq!(context.fourth_entry, i + 1);

        jump_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &FIRST_STATE));
        assert_eq!(context.fourth_exit, i + 1);
        assert_eq!(context.third_exit, i + 1);
        assert_eq!(context.fourth_action, i + 1);
        assert_eq!(context.first_entry, i + 2);
    }
}
