use actix_web::{http::StatusCode, test, App};
use cortex_staking_api::app;

#[actix_web::test]
async fn admin_route_rejects_missing_key() {
    let app = test::init_service(App::new().configure(app::configure_app)).await;

    let req = test::TestRequest::get()
        .uri("/admin/health")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn admin_route_rejects_partner_key() {
    let app = test::init_service(App::new().configure(app::configure_app)).await;

    let req = test::TestRequest::get()
        .uri("/admin/health")
        .insert_header(("Authorization", "Bearer partner-dev-key"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn admin_route_accepts_cortex_key() {
    let app = test::init_service(App::new().configure(app::configure_app)).await;

    let req = test::TestRequest::get()
        .uri("/admin/health")
        .insert_header(("Authorization", "Bearer cortex-dev-key"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn monad_route_accepts_partner_key() {
    let app = test::init_service(App::new().configure(app::configure_app)).await;

    let req = test::TestRequest::get()
        .uri("/monad/health")
        .insert_header(("Authorization", "Bearer partner-dev-key"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}