use serde::{Deserialize, Serialize};

// Should be moved (and I thought I did, but apparently not)
// Will be fixed sprint 2
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    otp: String,
}

pub mod email {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RequestBody {
        pub email_address: String,
    }
}
