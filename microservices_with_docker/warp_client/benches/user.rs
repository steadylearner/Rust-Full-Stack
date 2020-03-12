extern crate criterion;
use criterion::{
    // black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

// Test($cargo bench) first while the microservices aren't running to compare the performance later.
// Then, test it again and you will see that get_user shows obviously better results.
// list_users have more results that takes less time but there are more results that take more time. 
extern crate reqwest;
// use std::collections::HashMap;

async fn reqwest_rust_lang() -> Result<(), reqwest::Error> {
    let target = "https://www.rust-lang.org/";
    let res = reqwest::get(target).await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

async fn reqwest_single_page_app() -> Result<(), reqwest::Error> {
    let target = "https://www.steadylearner.com";
    let res = reqwest::get(target).await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

async fn reqwest_template() -> Result<(), reqwest::Error> {
    let target = "https://www.steadylearner.com/undefined";
    let res = reqwest::get(target).await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

async fn get_user() -> Result<(), reqwest::Error> {
    let target = "http://0.0.0.0:8000/api/user/v1/steadylearner";
    let res = reqwest::get(target).await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

async fn list_user() -> Result<(), reqwest::Error> {
    let target = "http://0.0.0.0:8000/api/user/v1";
    let res = reqwest::get(target).await?;

    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

fn bench_reqwest_rust_lang(c: &mut Criterion) {
    c.bench_function("request static website", |b| b.iter(|| reqwest_rust_lang()));
}

fn bench_reqwest_single_page_app(c: &mut Criterion) {
    c.bench_function("request single page app", |b| b.iter(|| reqwest_single_page_app()));
}

fn bench_reqwest_template(c: &mut Criterion) {
    c.bench_function("request template", |b| b.iter(|| reqwest_template()));
}

fn bench_get_user(c: &mut Criterion) {
    c.bench_function("get user", |b| b.iter(|| get_user()));
}

fn bench_list_user(c: &mut Criterion) {
    c.bench_function("list user", |b| b.iter(|| list_user()));
}

criterion_group!(
    benches,
    bench_reqwest_rust_lang,
    bench_reqwest_single_page_app,
    bench_reqwest_template,
    bench_get_user, bench_list_user
);
criterion_main!(benches);




