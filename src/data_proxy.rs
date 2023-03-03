pub struct Bucket {
    pub token: String,
    pub id: String
}

pub fn test() {
    println!("test called");
}

pub fn get_stat(bucket: Bucket) {
    println!("get_stat called");
}