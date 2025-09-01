use plyne::define_tasks;

define_tasks! {
    ResolvePostsSystem,
    pipelines {
        UserPipeline: u32,
        PostPipeline: u32,
    }
    vars {
        Dataset: String,
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

async fn get_users_by_const(user_pipeline: UserPipelineInput) {
    user_pipeline.send(10).unwrap();
}

async fn get_data_from_dataset(
    dataset: Dataset,
    user_pipeline: UserPipelineInput,
    post_pipeline: PostPipelineInput,
) {
    assert_eq!(*dataset, "Dataset");
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    [10, 30]
        .into_iter()
        .for_each(|user| user_pipeline.send(user).unwrap());
    [15, 55]
        .into_iter()
        .for_each(|post| post_pipeline.send(post).unwrap());
}

async fn fetch_posts_by_users(
    mut user_pipeline: UserPipelineOutput,
    post_pipeline: PostPipelineInput,
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

async fn consume_posts(mut post_pipeline: PostPipelineOutput) {
    while let Some(post) = post_pipeline.recv().await {
        assert!(post.is_multiple_of(5));
    }
}
