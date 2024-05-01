use leptos::*;
use app::*;

mod app;

fn main() {
    // Console error hook so panic stack trace can be viewed. 
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    
    leptos::mount_to_body(|| view! { <App/> })
}
