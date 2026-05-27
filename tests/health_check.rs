use std::net::TcpListener;

//let's construct the spawn_app function. This spawn function was later modified because we want a program with more port variability

fn spawn_app() -> String {

    let listener= TcpListener :: bind("127.0.0.1:0")
        .expect("Failed to bind random port");
  // how to retrieve the port used by by the OS
let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    // let's print the port name on the screen using "format!"" debug
format!("http://127.0.0.1:{}", port)

}


//We proceed to write the tests. We first invoke the [tokio::test]. 
#[tokio::test]



async fn health_check_works(){

  // we call in the spawn_app. We are to note that the health_check_works function calls in the spawn_app function's return. The spawn_app function itself is defined later on.
// we later made modifications so that we don't just call spawn_app which was just a function that didn't have a clear return. Now after 
//modifications, it has  a clear string return: the port that the OS chosen. So we set address to be the return of swap_app()
// a move will occur as the string is shifted from the function's output to the variable "address".. Take note of due referencing used.
let address =spawn_app();
  
  //let's proceed to bring in the reqwest method to perform Http requests against our appliocation
  //let's instantiate the client variable as an instance of the Client struct

  let client = reqwest::Client::new();

  //instantiate the response variable

  let response = client
     //Here we modify the get from 8000 to the one used by the 0S coz we use a random port 0 system... essentially the the http core plus the port name
     .get(&format!("{}/health_check", &address) )
     .send()
     .await
     .expect("Failed to execute Request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){

   // now we reuse the spawn_app function for this test segment
  let app_address=spawn_app();
  let client=reqwest::Client::new();
  let body=  "name=le%20guin&email=ursula_le_guin%40gmail.com";
  let response=client
      .post(&format!("{}/subscribe", &app_address))
      .header("Content-Type", "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
      .expect("Failed to execute request.");
    
    assert_eq!(200, response.status().as_u16());



}

//This is the second subtest, and this concerns returninga  400 when some data is missing

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
//let's construct the app_address variable to be equal to the output of the spawn_app function... That is it calls an instance of the spawn_app as spawn_app()
       let app_address = spawn_app();
       let client = reqwest::Client::new();
       let test_cases =   vec![("name=le%20guin", "missing the email"),
       ("email=ursula_le_guin%40gmail.com", "missing the name"),
       ("", "missing both name and email")
        ];


    for (invalid_body, error_message) in test_cases {

      let response = client
         .post(&format!("{}/subscribe", &app_address))
         .header("Content-Type", "application/x-www-form-urlencoded")
         .body(invalid_body)
         .send()
         .await
         .expect("Failed to execute request.");

        assert_eq!(
          400, response.status().as_u16(),
          "The API did not fail with 400 Bad Request when the payload was {}.",
          error_message
        
        );

    }

   
}




