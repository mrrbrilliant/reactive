use leptos_reactive::*;

#[derive(Default, Clone, Debug)]
struct GlobalState {
    count: u32,
    name: String,
}

fn main() {
    // creates a new reactive runtime
    // this is omitted from most of the examples in the docs
    // you usually won't need to call it yourself
    let runtime = create_runtime();

    // some global state with independent fields

    let state = create_rw_signal(GlobalState::default());

    #[rustfmt::skip]
    let (count, set_count) = create_slice(
        state,
        |state| state.count,
        |state, n| state.count = n,
    );

    #[rustfmt::skip]
    let (name, set_name) = create_slice(
        state,
        |state| state.name.clone(),
        |state, n| state.name = n,
    );

    create_effect(move |_| {
        // note: in the browser, use leptos::log! instead
        println!("name is {}", name.get());
    });
    create_effect(move |_| {
        println!("count is {}", count.get());
    });

    // setting count only causes count to log, not name
    set_count.set(42);

    // setting name only causes name to log, not count
    set_name.set("Bob".into());

    runtime.dispose();
}
