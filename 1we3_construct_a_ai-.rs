use serde::{Deserialize, Serialize};

// Configuration for AI-powered DevOps pipeline dashboard
#[derive(Debug, Serialize, Deserialize)]
struct AiDevOpsConfig {
    // General settings
    dashboard_title: String,
    refresh_interval: u64, // in seconds

    // API connections
    api_url: String,
    api_token: String,

    // Data sources
    data_sources: Vec<DataSource>,

    // Machine learning models
    models: Vec<Model>,

    // Pipeline configuration
    pipeline: Pipeline,
}

// Data source configuration
#[derive(Debug, Serialize, Deserialize)]
struct DataSource {
    name: String,
    url: String,
    credentials: Credentials,
}

// Credentials for data source
#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

// Machine learning model configuration
#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    model_type: String, // e.g. linear_regression, decision_tree, etc.
    training_data: Vec<TrainingData>,
}

// Training data for machine learning model
#[derive(Debug, Serialize, Deserialize)]
struct TrainingData {
    input: Vec<f64>,
    output: Vec<f64>,
}

// Pipeline configuration
#[derive(Debug, Serialize, Deserialize)]
struct Pipeline {
    stages: Vec<PipelineStage>,
}

// Pipeline stage configuration
#[derive(Debug, Serialize, Deserialize)]
struct PipelineStage {
    name: String,
    script: String, // script to execute in this stage
    dependencies: Vec<String>, // dependencies for this stage
}

// Default configuration
const DEFAULT_CONFIG: AiDevOpsConfig = AiDevOpsConfig {
    dashboard_title: "AI-powered DevOps Pipeline Dashboard".to_string(),
    refresh_interval: 60, // 1 minute

    api_url: "https://api.example.com".to_string(),
    api_token: "my_secret_token".to_string(),

    data_sources: vec![
        DataSource {
            name: "GitHub API".to_string(),
            url: "https://api.github.com".to_string(),
            credentials: Credentials {
                username: "my_username".to_string(),
                password: "my_password".to_string(),
            },
        },
    ],

    models: vec![
        Model {
            name: "Linear Regression".to_string(),
            model_type: "linear_regression".to_string(),
            training_data: vec![
                TrainingData {
                    input: vec![1.0, 2.0, 3.0],
                    output: vec![2.0, 4.0, 6.0],
                },
            ],
        },
    ],

    pipeline: Pipeline {
        stages: vec![
            PipelineStage {
                name: "Build".to_string(),
                script: "cargo build".to_string(),
                dependencies: vec![],
            },
            PipelineStage {
                name: "Test".to_string(),
                script: "cargo test".to_string(),
                dependencies: vec!["Build".to_string()],
            },
        ],
    },
};