use serde::*;
use tokio::fs;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    demo_panic().await?;
    Ok(())
}

/// ```bash
/// # show version
/// cargo tree | grep "serde_yml\|serde"
/// # outputs:
/// │   └── serde v1.0.214
/// │       └── serde_derive v1.0.214 (proc-macro)
/// ├── serde v1.0.214 (*)
/// ├── serde_json v1.0.132
/// │   └── serde v1.0.214 (*)
/// ├── serde_yml v0.0.12
/// │   └── serde v1.0.214 (*)
/// ```
/// Running this results with a message:
/// ```
/// attempt to deserialize Test
/// thread 'main' panicked at /**********/libyml-0.0.5/src/scanner.rs:2798:17:
/// String join would overflow memory bounds
/// ```
pub async fn demo_panic() -> Result<(), Box<dyn std::error::Error>> {
    use serde_yml::Value;

    #[derive(Serialize, Deserialize)]
    pub struct Test {
        pub content: String,
    }

    let t = Test {
        // NOTE: the amount of spaces at the end matters
        content: "\n    a {\n        ".into(),
    };

    // create the file with the failing string
    // ensure to deserialize using serde_yml

    fs::write("./generated-test.yaml", serde_yml::to_string(&t)?).await?;
    let bad_file = fs::read("./generated-test.yaml").await?;
    println!("attempt to deserialize Test");

    // PANICS // 
    let _: Result<Test, _> = serde_yml::from_slice(&bad_file);
    println!("attempt to deserialize serde_yml::Value");

    // PANICS // 
    let _: Result<Value, _> = serde_yml::from_slice(&bad_file);
    Ok(())
}
