#[tokio::test]
fn health_checks_works() {
    //arrange
    spawn_app().expect("failed to spawn our app.")
    //We need to bring in 'reqwest'
    //to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client.get("http://127.0.0.1:8000/health_check").send().expect("Failed to execute request.");

    //Assert 
    assert!(response.status().is_success());

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

//Launch our application in the background somehow.
async fn spawn_app() -> Result<() , std::io::Error> {
  todo!()
}
