use super::Msg;
use async_trait::async_trait;

#[async_trait]
pub trait AssembledComponent {
    async fn update(&mut self, msg: Msg);
}
