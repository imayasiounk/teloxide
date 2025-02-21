use std::{
    fmt::Debug,
    marker::{Send, Sync},
    sync::Arc,
};

use futures::future::BoxFuture;
use log::{log_enabled, trace, Level::Trace};

use crate::dispatching::dialogue::Storage;

/// Storage wrapper for logging purposes
///
/// Reports about any dialogue update or removal action on `trace` level
/// using `log` crate.
pub struct TraceStorage<S> {
    inner: Arc<S>,
}

impl<S> TraceStorage<S> {
    #[must_use]
    pub fn new(inner: Arc<S>) -> Arc<Self> {
        Arc::new(Self { inner })
    }

    pub fn into_inner(self) -> Arc<S> {
        self.inner
    }
}

impl<S, D> Storage<D> for TraceStorage<S>
where
    D: Debug,
    S: Storage<D> + Send + Sync + 'static,
{
    type Error = <S as Storage<D>>::Error;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static,
    {
        trace!("Removing dialogue with {}", chat_id);
        <S as Storage<D>>::remove_dialogue(self.inner.clone(), chat_id)
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static,
    {
        if log_enabled!(Trace) {
            Box::pin(async move {
                let to = format!("{:#?}", dialogue);
                let from =
                    <S as Storage<D>>::update_dialogue(self.inner.clone(), chat_id, dialogue)
                        .await?;
                trace!("Updated dialogue with {}, {:#?} -> {}", chat_id, from, to);
                Ok(from)
            })
        } else {
            <S as Storage<D>>::update_dialogue(self.inner.clone(), chat_id, dialogue)
        }
    }
}
