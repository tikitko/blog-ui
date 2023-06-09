use gloo_net::http::{Request, Response};
use gloo_net::Error;

#[async_trait(?Send)]
pub trait RequestableItem<P>: Sized {
    async fn request(params: P) -> Result<Request, Error>;
    async fn response(response: Response) -> Result<Self, Error>;
}

#[async_trait(?Send)]
pub trait Get<P>: Sized {
    async fn get(params: P) -> Result<Self, Error>;
}

#[async_trait(?Send)]
impl<I, P> Get<P> for I
where
    I: RequestableItem<P> + 'static,
    P: 'static,
{
    async fn get(params: P) -> Result<Self, Error> {
        Ok(Self::response(Self::request(params).await?.send().await?).await?)
    }
}
