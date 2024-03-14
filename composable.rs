use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

mod manager {
    pub mod account {
        use super::super::Actions;
        // Ví dụ cấu trúc dữ liệu cụ thể
        #[derive(Clone, Debug)]
        pub struct Account {
            pub username: String,
            pub balance: f64,
        }

        impl Actions for Account {
            fn create(&mut self) {
                // Logic tạo Account
            }

            fn update(&mut self) {
                // Logic cập nhật Account
            }

            fn delete(&mut self) {
                // Logic xóa Account
            }
        }
    }
}
// Định nghĩa trait DataActions cho các hành động dữ liệu
trait Actions {
    fn create(&mut self);
    fn update(&mut self);
    fn delete(&mut self);
}

// Định nghĩa State với generics T
#[derive(Debug, Clone)]
struct State<T> {
    shared: T,
    is_loading: bool,
    error: Option<String>,
    _marker: PhantomData<*const T>,
}

impl<T> State<T> {
    fn new(shared: T) -> Self {
        State {
            shared,
            is_loading: false,
            error: None,
            _marker: PhantomData,
        }
    }
}

// Định nghĩa các Action có thể có
#[derive(Debug)]
enum Action<T>
where
    T: Actions,
{
    Create(T),
    Update(T),
    Delete(T),
}

// Định nghĩa Middleware
type Middleware<T> = dyn Fn(&Action<T>, &State<T>) + Send + Sync;

// Định nghĩa Reducer
fn reducer<T: Actions + Clone>(state: &State<T>, action: Action<T>) -> State<T> {
    let mut new_state = state.clone();
    match action {
        Action::Create(mut data) => {
            data.create();
            new_state.shared = data;
        }
        Action::Update(mut data) => {
            data.update();
            new_state.shared = data;
        }
        Action::Delete(mut data) => {
            data.delete();
            new_state.shared = data;
        }
    }
    new_state
}

// Định nghĩa Store
struct Store<T: Actions + Clone + 'static> {
    state: Arc<State<T>>,
    reducer: Box<dyn Fn(&State<T>, Action<T>) -> State<T> + Send + Sync>,
    middlewares: Vec<Box<Middleware<T>>>,
    plugins: Vec<Box<dyn Plugin<T>>>,
}

impl<T: Actions + Clone + 'static> Store<T> {
    fn new(initial_state: T) -> Self {
        Store {
            state: Arc::new(State::new(initial_state)),
            reducer: Box::new(reducer::<T>),
            middlewares: Vec::new(),
            plugins: Vec::new(),
        }
    }

    fn dispatch(&mut self, action: Action<T>) {
        for middleware in &self.middlewares {
            middleware(&action, &self.state);
        }
        for plugin in &self.plugins {
            plugin.on_action(&action, &self.state);
        }
        let new_state = (self.reducer)(&self.state, action);
        self.state = Arc::new(new_state);
    }

    fn add_middleware(&mut self, middleware: Box<Middleware<T>>) {
        self.middlewares.push(middleware);
    }

    fn add_plugin<P: Plugin<T> + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }
}

// Định nghĩa Database
struct Database<T: Actions + Clone + 'static> {
    storage: HashMap<String, T>,
}

impl<T: Actions + Clone + 'static> Database<T> {
    fn new() -> Self {
        Database {
            storage: HashMap::new(),
        }
    }
}

// Định nghĩa Effect
struct Effect<T: Actions + Clone + 'static> {
    operation: Box<dyn FnMut(&mut Database<T>, &mut State<T>) + Send>,
}

impl<T: Actions + Clone + 'static> Effect<T> {
    fn apply(&mut self, database: &mut Database<T>, state: &mut State<T>) {
        (self.operation)(database, state);
    }
}

// Định nghĩa Logger
fn logger<T: Actions + std::fmt::Debug + 'static>(action: &Action<T>, state: &State<T>) {
    println!("Action: {:?}, State: {:?}", action, state);
}

// Định nghĩa trait Plugin với generics
trait Plugin<T>
where
    T: Actions + Clone + 'static,
{
    fn name(&self) -> String;
    fn on_action(&self, action: &Action<T>, state: &State<T>);
    fn as_any(&self) -> &dyn Any;
}

// Ví dụ về cách sử dụng plugin với cấu trúc dữ liệu khác nhau
#[derive(Debug)]
struct Logging;

impl<T: Actions + Clone + std::fmt::Debug + 'static> Plugin<T> for Logging {
    fn name(&self) -> String {
        "Logging".to_string()
    }

    fn on_action(&self, action: &Action<T>, state: &State<T>) {
        println!("Logging action: {:?}, state: {:?}", action, state);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Hàm async để xử lý các tác vụ bất đồng bộ
async fn async_action_handler<T: Actions + Clone + 'static>(
    mut effect: Effect<T>,
    database: &mut Database<T>,
    state: &mut State<T>,
) {
    effect.apply(database, state);
}

#[tokio::main]
async fn main() {
    let mut store = Store::new(manager::account::Account {
        username: "user".to_string(),
        balance: 0.0,
    });
    let effect = Effect {
        operation: Box::new(|_, _| {}),
    }; // Khởi tạo effect tùy thuộc vào logic của bạn
    let mut database = Database::new(); // Khởi tạo database
    let mut state = State::new(manager::account::Account {
        username: "user".to_string(),
        balance: 0.0,
    }); // Khởi tạo state

    // Thêm middleware cho logging
    store.add_middleware(Box::new(|action, state| {
        logger(action, state);
    }));
    
    // Thêm plugin cho Logging
    store.add_plugin(Logging);

    // Tạo các action và dispatch chúng
    let action = Action::Create(manager::account::Account {
        username: "new_user".to_string(),
        balance: 0.0,
    });

    // Sử dụng async_action_handler để xử lý bất đồng bộ
    async_action_handler(effect, &mut database, &mut state).await;

    store.dispatch(action); // Dispatch hành động bất đồng bộ
}

