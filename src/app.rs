use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::header::*;
use crate::components::search_field::*;
use crate::pages::*;
use crate::utils::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:slug")]
    Post { slug: String },
    #[at("/posts")]
    Posts,
    #[at("/posts/search")]
    PostsSearchRoot,
    #[at("/posts/search/:query")]
    PostsSearch { query: String },
    #[at("/authors/:slug")]
    Author { slug: String },
    #[at("/authors")]
    Authors,
    #[at("/authors/search")]
    AuthorsSearchRoot,
    #[at("/authors/search/:query")]
    AuthorsSearch { query: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn switch(route: Route) -> Html {
        match route {
            Route::Post { slug } => html! { <Post { slug } /> },
            Route::Home | Route::Posts => html! { <Posts />},
            Route::PostsSearchRoot => {
                html! { <Search mode={ SearchMode::Posts { query: None } } />}
            }
            Route::PostsSearch { query } => {
                html! { <Search mode={ SearchMode::Posts { query: Some(query) } } />}
            }
            Route::Author { slug } => html! { <Author { slug } /> },
            Route::Authors => html! { <Authors /> },
            Route::AuthorsSearchRoot => {
                html! { <Search mode={ SearchMode::Authors { query: None } } />}
            }
            Route::AuthorsSearch { query } => {
                html! { <Search mode={ SearchMode::Authors { query: Some(query) } } />}
            }
            Route::NotFound => html! { <PageNotFound /> },
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let logged_user = use_reducer(|| Default::default());
    html! {
        <BrowserRouter>
            <ContextProvider<LoggedUserContext> context={logged_user}>
                <Header />
                <Body />
            </ContextProvider<LoggedUserContext>>
        </BrowserRouter>
    }
}

pub fn app_renderer() -> yew::Renderer<impl BaseComponent> {
    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element)
}
