use rocket::{get, post, form::Form};
use rocket_auth::{Error, Auth, Signup, Login};

#[post("/signup", data = "<form>")]
pub async fn signup(form: Form<Signup>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok("You signed up.")
}

#[post("/login", data = "<form>")]
pub async fn login(form: Form<Login>, auth: Auth<'_>) -> Result<&'static str, Error> {
    auth.login(&form).await?;
    Ok("You're logged in.")
}

#[get("/logout")]
pub fn logout(auth: Auth<'_>) -> Result<&'static str, Error> {
    match auth.logout() {
        Ok(_) => Ok("You're logged out."),
        Err(e) => Err(e),
    }
}

#[get("/status")]
pub async fn status(auth: Auth<'_>) -> &'static str {
    if auth.get_user().await.is_some() {
        "logged"
    } else {
        "not logged"
    }
}


/*
EX:
curl -X POST http://localhost:8000/signup --data "username=test&password=Test1234&email=mail@mail.ex"
curl -X POST http://localhost:8000/login --data "username=test&password=Test1234&email=mail@mail.ex" -c /tmp/cookies.txt; echo ""; cat /tmp/cookies.txt
curl http://localhost:8000/logout -b /tmp/cookies.txt
curl http://localhost:8000/status -b /tmp/cookies.txt
*/