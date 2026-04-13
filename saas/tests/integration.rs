use floz::testing::TestApp;

#[tokio::test]
async fn test_health_endpoint() {
    let app = TestApp::new().await;
    let resp = app.get("/health").send().await;

    assert_eq!(resp.status(), 200, "Health check should return 200 OK");
}
