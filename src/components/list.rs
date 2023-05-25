use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::*;
use crate::hash_map_context::*;
use crate::get::*;

use crate::Route;

pub struct ExternalListContainerParams<P: Clone + PartialEq> {
    pub custom_params: P,
    pub limit: u64,
    pub skip: u64,
}

#[async_trait(?Send)]
pub trait ExternalListContainer: Clone + PartialEq {
    type Item: Clone + PartialEq + KeyedItem<Key = u64>;
    fn items(&self) -> Vec<Self::Item>;
    fn total(&self) -> u64;
    fn skip(&self) -> u64;
    fn limit(&self) -> u64;
}

#[derive(Properties, PartialEq)]
pub struct ListProps<C, P = ()>
where
    C: ExternalListContainer + RequestableItem<ExternalListContainerParams<P>> + 'static,
    P: Clone + PartialEq + 'static,
{
    pub params: P,
    #[prop_or(10)]
    pub items_per_page: u64,
    pub route_to_page: Route,
    pub component: Callback<Option<C::Item>, Html>,
}

#[function_component(List)]
pub fn list<C, P = ()>(props: &ListProps<C, P>) -> Html
where
    C: ExternalListContainer + RequestableItem<ExternalListContainerParams<P>> + 'static,
    P: Clone + PartialEq + 'static,
{
    let items_cache = use_context::<HashMapContext<u64, C::Item>>();

    let page = use_location()
        .unwrap()
        .query::<PageQuery>()
        .map(|it| it.page)
        .unwrap_or(1);
    let params = props.params.clone();
    let limit = props.items_per_page;
    let skip = (page - 1) * limit;
    let route_to_page = props.route_to_page.clone();
    
    let list_container = use_state_eq(|| None);
    {
        let items_cache = items_cache.clone();
        let list_container = list_container.clone();
        use_effect_with(page, move |_| {
            let items_cache = items_cache.clone();
            let list_container = list_container.clone();
            list_container.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_list_container = C::get(ExternalListContainerParams { custom_params: params, limit, skip }).await.unwrap();
                if let Some(items_cache) = items_cache {
                    items_cache.dispatch(ReducibleHashMapAction::Batch(fetched_list_container.items()))
                }
                list_container.set(Some(fetched_list_container));
            });
            || ()
        });
    }

    let Some(list_container) = (*list_container).clone() else {
        return (0..limit).map(|_| {
            props.component.emit(Option::None)
        }).collect::<Html>()
    };
    let total_pages = (list_container.total() as f64 / limit as f64).ceil() as u64;
    html! {
        <>
            {
                list_container.items().into_iter().map(|item| {
                    props.component.emit(Option::Some(item))
                }).collect::<Html>()
            }
            if total_pages > 1 {
                <Pagination
                    { page }
                    { total_pages }
                    { route_to_page }
                />
            }
        </>
    }
}