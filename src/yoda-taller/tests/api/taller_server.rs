use crate::swapi_mock::person_query_result;
use yoda_taller::swapi::Person;

use crate::helpers::TestApp;

#[tokio::test]
async fn yoda_is_not_taller_than_himself() {
    let app = TestApp::spawn().await;
    let name = "Yoda";

    let yoda_mock = Person {
        name: name.to_string(),
        height: "66".to_string(),
    };
    let body = person_query_result(&yoda_mock);
    app.swapi_server.mock_people_query(name, body).await;
    let response = app.send_taller_req(name).await;

    assert_eq!(200, response.status().as_u16());
}