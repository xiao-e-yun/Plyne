use tokio::sync::oneshot;

use crate::{Input, Output};

pub type Function<Args, Ret> = Input<FnEevent<Args, Ret>>;
pub type FunctionHandler<Args, Ret> = Output<FnEevent<Args, Ret>>;
pub type FnEevent<Args, Ret> = (Args, oneshot::Sender<Ret>);

pub trait FnPipeline<Args, Ret> {
    fn call(&self, args: Args) -> impl Future<Output = Ret>;
}

impl<Args, Ret> FnPipeline<Args, Ret> for Function<Args, Ret> {
    async fn call(&self, args: Args) -> Ret {
        let (sender, receiver) = oneshot::channel();
        self.send((args, sender)).unwrap();
        receiver.await.unwrap()
    }
}
