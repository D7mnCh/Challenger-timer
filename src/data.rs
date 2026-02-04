#[derive(Debug)]
pub enum Session {
    Work,
    Rest,
}
#[derive(Debug)]
pub struct Data {
    // for this field gonna reset secs
    pub reset_with_new_user_input: bool,
    //reset: bool,
    pub pause: bool,
    pub instant: std::time::Instant,
    pub session: Session,

    pub rest_secs: u64,
    pub work_secs: u64,
}
