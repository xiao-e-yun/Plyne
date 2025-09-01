pub mod pipeline;

pub use pipeline::*;

pub use magic_params;
pub use paste;

#[macro_export]
macro_rules! define_tasks {
    (
        $tasks_name: ident $(,)?
        pipelines {
            $($name: ident: $ty: ty),* $(,)?
        } $(,)?
        vars {
            $($var: ident: $vty: ty),* $(,)?
        } $(,)?
        tasks {
            $($task: ident),* $(,)?
        } $(,)?
    ) => {
        // Pipelines
        $crate::define_types!(pipelines $($name: $ty),*);

        // Vars
        $crate::define_types!(vars $($var: $vty),*);

        $crate::paste::paste! {
            // Context
            $crate::magic_params::define_context!([< $tasks_name Context >] {
                $( [< $name:snake >]: $name,)*
                $( [< $var:snake >]: $var,)*
            });
            
            $(
                impl [< From $tasks_name Context >] for [< $name Input >] {
                    fn from_context(ctx: &[< $tasks_name Context >]) -> Self { [< $name Input >](ctx.[< $name:snake >].input().unwrap()) }
                }

                impl [< From $tasks_name Context >] for [< $name Output >] {
                    fn from_context(ctx: &[< $tasks_name Context >]) -> Self { [< $name Output >](ctx.[< $name:snake >].output().unwrap()) }
                }
            )*

            $crate::magic_params::context_as_params!([< $tasks_name Context >]);
        
            // System
            struct $tasks_name([< $tasks_name Context >]);

            impl $tasks_name {
                #[allow(dead_code)]
                pub fn new($([< $var:snake >]: $vty),*) -> Self {
                    Self([< $tasks_name Context >] {
                        $( [< $name:snake >]: $name(std::sync::Arc::new($crate::Pipeline::new(false))),)*
                        $( [< $var:snake >]: $var([< $var:snake >]),)*
                    })
                }

                #[allow(dead_code)]
                pub async fn execute(self) {
                    futures::join!( $( $task.call(&self.0), )* );
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_types {
    (pipelines $($name: ident: $ty: ty),*) => {
        $crate::paste::paste! { $(
            #[derive(Debug, Clone)]
            struct $name(std::sync::Arc<$crate::Pipeline<$ty>>);
            struct [< $name Input >](tokio::sync::mpsc::UnboundedSender<$ty>);
            struct [< $name Output >](tokio::sync::mpsc::UnboundedReceiver<$ty>);

            $crate::define_types!(deref $name: $crate::Pipeline<$ty>);
            $crate::define_types!(deref [< $name Input >]: tokio::sync::mpsc::UnboundedSender<$ty>);
            $crate::define_types!(deref [< $name Output >]: tokio::sync::mpsc::UnboundedReceiver<$ty>);

            impl std::ops::DerefMut for [< $name Output >] {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )* }
    };
    (vars $($var: ident: $vty: ty),*) => {
        $crate::paste::paste! { $(
            #[derive(Debug, Clone)]
            struct $var($vty);

            $crate::define_types!(deref $var: $vty);

            impl std::ops::DerefMut for $var {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )* }
    };
    (deref $name: ident: $ty: ty) => {
        $crate::paste::paste! {
            impl std::ops::Deref for $name {
                type Target = $ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    };
}
