use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::svg_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Option<Post>,
    pub is_full: bool,
    pub link_to: bool,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let PostCardProps {
        post,
        is_full,
        link_to,
    } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let main_content = html! {
        <>
            <div
                style={
                    format!(
                        "height:194px;width:100%;--image-url:url({});",
                        post
                            .as_ref()
                            .map(|p| p.image_url.clone())
                            .flatten()
                            .unwrap_or_default()
                    )
                }
                class="img-block bd-placeholder-img"
                role="img"
            />
            <div class="card-body">
                <h5 class="card-title placeholder-glow">
                    if let Some(title) = post.as_ref().map(|p| p.title.clone()) {
                        { title }
                    } else {
                        <span class="placeholder col-6 bg-secondary"></span>
                    }
                </h5>
                <article class="card-text placeholder-glow">
                    if let Some(text) = post.as_ref().map(|post| {
                        if let (Some(content), true) = (post.content.clone(), is_full) {
                            Html::from_html_unchecked(AttrValue::from(content))
                        } else {
                            html! { post.summary.clone() }
                        }
                    }) {
                        { text }
                    } else {
                        <span class="placeholder col-7 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-6 bg-secondary"></span> { " " }
                        <span class="placeholder col-6 bg-secondary"></span> { " " }
                        <span class="placeholder col-5 bg-secondary"></span> { " " }
                        <span class="placeholder col-5 bg-secondary"></span> { " " }
                        <span class="placeholder col-3 bg-secondary"></span> { " " }
                        if is_full {
                            <span class="placeholder col-7 bg-secondary"></span> { " " }
                            <span class="placeholder col-4 bg-secondary"></span> { " " }
                            <span class="placeholder col-4 bg-secondary"></span> { " " }
                            <span class="placeholder col-6 bg-secondary"></span> { " " }
                            <span class="placeholder col-6 bg-secondary"></span> { " " }
                            <span class="placeholder col-5 bg-secondary"></span> { " " }
                            <span class="placeholder col-5 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                        }
                    }
                </article>
            </div>
        </>
    };
    html! {
        <div class="card mb-3">
            <div class="card-header placeholder-glow border-0">
                <div class="row align-items-center">
                    <div class="d-flex col align-items-center justify-content-start" style="height: 24px;">
                        <img
                            width="24"
                            height="24"
                            src={
                                post
                                    .as_ref()
                                    .map(|c| c.author.image_url())
                                    .unwrap_or_default()
                            }
                            alt={
                                post
                                    .as_ref()
                                    .map(|c| c.author.slug.clone())
                            }
                            class="img-block rounded me-1"
                        />
                        if let Some(post) = &post {
                            <Link<Route, (), Author>
                                classes={ classes!("text-decoration-none") }
                                to={ Route::Author { slug: post.author.slug.clone() } }
                                state={ Some(post.author.clone()) }
                            >
                                <strong>
                                    { &post.author.slug }
                                </strong>
                            </Link<Route, (), Author>>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                    <div class="d-flex col align-items-center justify-content-end" style="height: 24px;">
                        if let Some(post) = &post {
                            <small> { date::format(post.created_at) } </small>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                </div>
            </div>
            if let (Some(post), true) = (post.as_ref(), link_to) {
                <Link<Route, (), Post>
                    classes="text-decoration-none"
                    to={ Route::Post { slug: post.slug.clone(), id: post.id } }
                    state={ Some(post.clone()) }
                >
                    { main_content }
                </Link<Route, (), Post>>
            } else {
                { main_content }
            }
            <div class="card-footer placeholder-glow">
                <div class="row align-items-center">
                    <div class="d-flex col align-items-center justify-content-start" style="height: 24px;">
                        if let Some(post) = post.as_ref() {
                            <p class="mt-0 mb-0">
                                { post.tags.iter().map(|tag| { html! {
                                    <>
                                        <Link<Route, (), Tag>
                                            classes="link-dark link-underline-opacity-25 link-underline-opacity-100-hover"
                                            to={ Route::Tag { id: tag.id, slug: tag.slug.clone() } }
                                            state={ Some(tag.clone()) }
                                        >
                                            { tag.title.clone() }
                                        </Link<Route, (), Tag>>
                                        { " " }
                                    </>
                                } }).collect::<Html>() }
                            </p>
                        } else {
                            <span class="placeholder col-6 bg-secondary"></span>
                        }
                    </div>
                    <div class="d-flex col align-items-center justify-content-end" style="height: 24px;">
                        if let (
                            Some(post),
                            LoggedUserState::ActiveAndLoaded { token: _, author },
                        ) = (
                            post.as_ref(),
                            logged_user_context.state.clone(),
                        ) {
                            if author.slug == post.author.slug {
                                <Link<Route, (), Post>
                                    classes="text-decoration-none"
                                    to={ Route::EditPost { id: post.id } }
                                    state={ Some(post.clone()) }
                                >
                                    <PencilSquareImg />
                                </Link<Route, (), Post>>
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
