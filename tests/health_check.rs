#[tokio::test]
async fn health_checks_works() {
    spawn_app();
    //No .await call, therefore no need for 'spawn_app' to be async now.
    //We are also running tests, so it is not worht it to propagate errors;
    //if we fail to perform the required setup we can just panic and crash
    //all the things.

    fn spawn_app() {
        let server = zero2prod::run().expect("Failed to bind address");
        //Launch the server as a background task
        //tokio::spawn returns a handle to the spawned future,
        //but we have no use for it here, hence the non-binding let
        let _ = tokio::spawn(server);
    }
}
