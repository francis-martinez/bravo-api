fn main() {
    dotenvy::dotenv().ok();

    println!(
        "cargo:rustc-env=VITE_IAM_PUBLIC_API_KEY={}",
        std::env::var("VITE_IAM_PUBLIC_API_KEY").expect("VITE_IAM_PUBLIC_API_KEY must be set")
    );
}
