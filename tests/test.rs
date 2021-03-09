use playwright::{
    api::{Browser, BrowserContext, BrowserType, Geolocation, Response},
    *
};

runtime_test!(hello, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
});

runtime_test!(awesome, {
    env_logger::builder().is_test(true).try_init().ok();
    let p = Playwright::initialize().await.unwrap(); // if drop all resources are disposed
    p.prepare().unwrap(); // install browsers
    let mut bt = p.chromium();
    let mut b = launch(&mut bt).await;
    let mut c = new_context(&mut b).await;
    let mut p = c.new_page().await.unwrap();
    let _response: Option<Response> = p.goto_builder("https://example.com/").goto().await.unwrap();
    //// let _ = p.main_frame().query_selector_all("a").await.unwrap();
    //// let _ = p.main_frame().title().await.unwrap();
    // let mut a = p.query_selector("a").await.unwrap().unwrap();
    // let v = a.get_attribute("href").await.unwrap();
    // dbg!(v);
    // p.go_back_builder().go_back().await.unwrap();
});

async fn launch(t: &mut BrowserType) -> Browser {
    t.launcher().headless(true).launch().await.unwrap()
}

async fn new_context(b: &mut Browser) -> BrowserContext {
    let a = "asdf".to_string();
    b.context_builder().user_agent(&a).build().await.unwrap()
}
