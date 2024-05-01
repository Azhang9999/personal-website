use  leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    view! {
        <div class="my-4 p-4 bg-gray-100 rounded-lg">
            <h2 class="text-2xl font-bold">Contact Information</h2>
            <ul class="mt-4 list-disc list-inside">
                <li class="mb-2">Email: <a href="mailto:your-email@example.com" class="text-blue-600">your-email@example.com</a></li>
                <li class="mb-2">Phone: <a href="tel:1234567890" class="text-blue-600">123-456-7890</a></li>
                <li class="mb-2">Address: 123 Main St, City, State, ZIP Code</li>
            </ul>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <a href="#contact">Go to Contact Section</a>
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                "Something's here | "
                {move || if count.get() == 0 {
                    "Click me!".to_string()
                } else {
                    count.get().to_string()
                }}
                " | Some more text"
            </button>
            
            
        </div>
        <div id="contact">
            <ContactInfo/>
        </div>
    }
}
