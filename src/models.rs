

// models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub person_id: i64,
    pub isim: String,
    pub soyisim: String,
    pub email: String,
    pub yas: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Animal{
    pub animal_id: i64,
    pub cins:String,
    pub cinsiyet:String,
    pub ayak_sayisi:i32,
}
