pub mod function;
pub mod pipeline;

pub use function::*;
pub use pipeline::*;

pub use magic_params;
pub use paste;

// Pipeline
pub type Input<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub type Output<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

#[macro_export]
macro_rules! define_tasks {
    (
        $tasks_name: ident
        pipelines {
            $($name: ident: $ty: ty),* $(,)?
        }
        vars {
            $($var: ident: $vty: ty),* $(,)?
        }
        tasks {
            $($task: ident),* $(,)?
        }
    ) => {
        $crate::paste::paste! {
            // Context
            $crate::magic_params::define_context!([< $tasks_name Context >] {
                $( $name: $crate::Pipeline<$ty>,)*
                $( $var: $vty,)*
            });

            // Pipelines
            $(
                impl [< From $tasks_name Context >]<'_> for $crate::Input<$ty> {
                    fn from_context(ctx: &[< $tasks_name Context >]) -> Self { ctx.$name.input().unwrap() }
                }

                impl [< From $tasks_name Context >]<'_> for $crate::Output<$ty> {
                    fn from_context(ctx: &[< $tasks_name Context >]) -> Self { ctx.$name.output().unwrap() }
                }
            )*

            $crate::magic_params::context_as_params!([< $tasks_name Context >], 12);

            // System
            struct $tasks_name([< $tasks_name Context >]);

            impl $tasks_name {
                #[must_use]
                #[allow(dead_code)]
                pub fn new($($var: $vty),*) -> Self {
                    Self([< $tasks_name Context >] {
                        $( $name: $crate::Pipeline::new(false), )*
                        $( $var, )*
                    })
                }

                #[allow(dead_code)]
                pub async fn execute(self) -> [< $tasks_name Context >] {
                    futures::join!( $( $task.call(&self.0), )* );
                    self.0
                }
            }
        }
    };
}
