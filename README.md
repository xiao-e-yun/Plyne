# Plyne
Based pipeline of async task system

- Simple
- Type Safe
- Zero Cost

# Usage
Install
```bash
cargo add plyne
```

Define tasks
```rust
use plyne::define_tasks;

define_tasks! {
    TaskSystem // Name of the task system
    pipelines { // Define pipelines
        APipeline: Vec<u8>, // you can use APipelineInput or APipelineOutput
    }
    vars { // Define variables
        Dataset: Arc<Vec<u8>>, // Avoid larger types
        Config: config::Config, // Avoid same name as struct
    }
    tasks { // Define tasks
        load_data,
        parse_data,
    }
}

async fn load_data(dataset: dataset, pipeline: APipelineInput) -> String {
    for data in dataset.chunks(8) {
        pipeline.send(chunk.to_vec());
    }
}

async fn parse_data(a_pipeline: APipelineOutput, config: Config) -> Vec<u8> {
    let mut results = String::new();
    while let Some(data) = a_pipeline.recv().await {
        ... // parse data using config
    }
}
```

Execute tasks
```rust
#[tokio::main]
async fn main() {
    TaskSystem::new(
        vec![13,31,31,32,15,32,14,15,22,22,66,23,17,61]
    ).execute().await;
}
```
