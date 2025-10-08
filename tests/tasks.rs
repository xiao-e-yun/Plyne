use plyne::{define_tasks, Input, Output};

pub struct Uncloneable;

define_tasks! {
    ResolvePostsSystem,
    pipelines {
        user_pipeline: i32,
        post_pipeline: u32,
    }
    vars {
        dataset: String,
    }
    tasks {
        get_users_by_const,
        get_data_from_dataset,
        fetch_posts_by_users,
        consume_posts,
    }
}

#[tokio::test]
async fn main() {
    ResolvePostsSystem::new("Dataset".to_string()).execute().await;
}

async fn get_users_by_const(user_pipeline: Input<i32>) {
    user_pipeline.send(10).unwrap();
}

async fn get_data_from_dataset(
    dataset: &String,
    user_pipeline: Input<i32>,
    post_pipeline: Input<u32>
) {
    assert_eq!(dataset, "Dataset");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    [10, 30]
        .into_iter()
        .for_each(|user| user_pipeline.send(user).unwrap());
    [15, 55]
        .into_iter()
        .for_each(|post| post_pipeline.send(post).unwrap());
}

async fn fetch_posts_by_users(
    mut user_pipeline: Output<i32>,
    post_pipeline: Input<u32>
) {
    while let Some(user) = user_pipeline.recv().await {
        match user {
            10 => [10]
                .into_iter()
                .for_each(|post| post_pipeline.send(post).unwrap()),
            30 => [30, 35, 40, 45, 50]
                .into_iter()
                .for_each(|post| post_pipeline.send(post).unwrap()),
            _ => unreachable!(),
        }
    }
}

async fn consume_posts(mut post_pipeline: Output<u32>) {
    while let Some(post) = post_pipeline.recv().await {
        assert!(post.is_multiple_of(5));
    }
}
