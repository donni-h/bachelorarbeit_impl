use bachelorarbeit::domain::models::metadata::UserName;
use bachelorarbeit::domain::models::metadata::SessionStatus;


fn main() {
    let user = UserName::new("Hannes");
    println!("{}", user);
    let status = SessionStatus::Expired;

    println!("{status}")
}
