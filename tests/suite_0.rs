use hsm;

struct Context {
    the_entry: usize,
    internal_action: usize,
    external_action: usize,
    the_exit: usize,
}

enum Event {
    Initial,
    Internal,
    External,
}

type Transition = hsm::Transition<Context, Event>;

type StateMachine = hsm::StateMachine<Context, Event>;

struct RootState;
struct InitialState;
struct TheState;

impl hsm::State<Context, Event> for RootState {}

impl hsm::State<Context, Event> for InitialState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn transition(&self, _context: &mut Context, _event: &Event) -> Transition {
        hsm::Transition::<Context, Event>::Local(&THE_STATE, None)
    }
}

impl hsm::State<Context, Event> for TheState {
    fn parent(&self) -> Option<&'static dyn hsm::State<Context, Event>> {
        Some(&ROOT_STATE)
    }

    fn entry(&self, context: &mut Context) {
        context.the_entry += 1;
    }

    fn transition(&self, context: &mut Context, event: &Event) -> Transition {
        match event {
            Event::Internal => {
                context.internal_action += 1;
                hsm::Transition::<Context, Event>::Internal(None)
            }
            Event::External => {
                context.external_action += 1;
                hsm::Transition::<Context, Event>::External(&THE_STATE, None)
            }
            _ => hsm::Transition::<Context, Event>::Unknown,
        }
    }

    fn exit(&self, context: &mut Context) {
        context.the_exit += 1;
    }
}

static ROOT_STATE: RootState = RootState;
static INITIAL_STATE: InitialState = InitialState;
static THE_STATE: TheState = TheState;

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
        the_entry: 0,
        internal_action: 0,
        external_action: 0,
        the_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &THE_STATE));
}

fn internal_step(machine: &mut StateMachine, context: &mut Context) {
    let internal_event = Event::Internal;
    machine.dispatch(context, &internal_event);
}

fn external_step(machine: &mut StateMachine, context: &mut Context) {
    let external_event = Event::External;
    machine.dispatch(context, &external_event);
}

#[test]
fn multi_self() {
    let mut context = Context {
        the_entry: 0,
        internal_action: 0,
        external_action: 0,
        the_exit: 0,
    };
    let mut machine = create_machine();
    assert!(core::ptr::eq(machine.active(), &INITIAL_STATE));

    initial_step(&mut machine, &mut context);
    assert!(core::ptr::eq(machine.active(), &THE_STATE));
    assert_eq!(context.the_entry, 1);

    for i in 0..1000 {
        internal_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THE_STATE));
        assert_eq!(context.the_exit, i);
        assert_eq!(context.internal_action, i + 1);
        assert_eq!(context.external_action, i);
        assert_eq!(context.the_entry, i + 1);

        external_step(&mut machine, &mut context);
        assert!(core::ptr::eq(machine.active(), &THE_STATE));
        assert_eq!(context.the_exit, i + 1);
        assert_eq!(context.internal_action, i + 1);
        assert_eq!(context.external_action, i + 1);
        assert_eq!(context.the_entry, i + 2);
    }
}
