use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    html! {
        <div class="d-grid gap-2">
            <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Home { "active" } else { "" })} to={ Route::Home }>{ "Главная" }</Link<Route>>
            <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Posts { "active" } else { "" })} to={ Route::Posts }>{ "Публикации" }</Link<Route>>
            <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Authors { "active" } else { "" })} to={ Route::Authors }>{ "Авторы" }</Link<Route>>
        </div>
    }
}
