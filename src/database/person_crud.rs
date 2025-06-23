use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{bson::Document, Client, Collection};
use futures::TryStreamExt;
use std::error::Error;
use crate::models::Person;
use std::io::{Write}; // flush() metodunu kullanabilmek için bu import gereklidir


pub async fn connect_to_db() -> Result<Client, Box<dyn Error>> {
    let uri = "mongodb://localhost:27017";
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("dbProject1".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn create_person(
    person_collection: &Collection<Person>,
    counters_collection: &Collection<Document>,
) -> mongodb::error::Result<i64> {
    let before_update = counters_collection.find_one(doc! { "_id": "person_id" }, None).await?;
    println!("Sayaç güncellemesinden önceki değer: {:?}", before_update);

    let update_options = mongodb::options::UpdateOptions::builder().upsert(true).build();
    counters_collection
        .update_one(doc! { "_id": "person_id" }, doc! { "$inc": { "seq": 1 } }, update_options)
        .await?;

    let after_update = counters_collection.find_one(doc! { "_id": "person_id" }, None).await?;
    println!("Sayaç güncellemesinden sonraki değer: {:?}", after_update);

    let next_person_id = match after_update {
        Some(doc) => match doc.get_i64("seq") {
            Ok(seq) => seq,
            Err(_) => doc.get_i32("seq").unwrap_or(0) as i64,
        },
        None => 0,
    };

    println!("Sonraki person_id: {}", next_person_id);

    let isim = read_input("İsim: ");
    let soyisim = read_input("Soyisim: ");
    let email = read_input("Email: ");
    let yas: i32 = read_input("Yaş: ").trim().parse().expect("Geçersiz yaş");

    let new_person = Person {
        person_id: next_person_id,
        isim,
        soyisim,
        email,
        yas,
    };

    person_collection.insert_one(new_person, None).await?;
    println!("Yeni kişi başarıyla eklendi");

    Ok(next_person_id)
}

pub async fn read_persons(person_collection: &Collection<Person>) -> mongodb::error::Result<()> {
    let cursor = person_collection.find(None, None).await?;
    let results: Vec<Person> = cursor.try_collect().await?;
    for person in results {
        println!("{:?}", person);
    }
    Ok(())
}

pub async fn update_person(
    person_collection: &Collection<Person>,
    person_id: i64,
) -> mongodb::error::Result<()> {
    let filter = doc! { "person_id": person_id };

    let yeni_isim = read_input("Yeni isim (boş bırakırsanız değişmeyecek): ");
    let yeni_soyisim = read_input("Yeni soyisim (boş bırakırsanız değişmeyecek): ");
    let yeni_email = read_input("Yeni email (boş bırakırsanız değişmeyecek): ");
    let yeni_yas = read_input("Yeni yaş (boş bırakırsanız değişmeyecek): ");

    let mut update_doc = doc! {};

    if !yeni_isim.trim().is_empty() {
        update_doc.insert("isim", yeni_isim);
    }
    if !yeni_soyisim.trim().is_empty() {
        update_doc.insert("soyisim", yeni_soyisim);
    }
    if !yeni_email.trim().is_empty() {
        update_doc.insert("email", yeni_email);
    }
    if let Ok(yas) = yeni_yas.trim().parse::<i32>() {
        update_doc.insert("yas", yas);
    }

    let update = doc! { "$set": update_doc };

    let update_result = person_collection.update_one(filter, update, None).await?;
    if update_result.matched_count > 0 {
        println!("Person_id {} başarıyla güncellendi.", person_id);
    } else {
        println!("Person_id {} için hiçbir belge bulunamadı.", person_id);
    }

    let updated_person = person_collection.find_one(doc! { "person_id": person_id }, None).await?;
    match updated_person {
        Some(person) => println!("Güncellenmiş kişi: {:?}", person),
        None => println!("Kişi bulunamadı"),
    }
    Ok(())
}

pub async fn delete_person(
    person_collection: &Collection<Person>,
    person_id: i64,
) -> mongodb::error::Result<()> {
    let filter = doc! { "person_id": person_id };
    let delete_result = person_collection.delete_one(filter, None).await?;
    println!("Silinen belge sayısı: {}", delete_result.deleted_count);
    Ok(())
}

pub async fn search_by_name(
    person_collection: &Collection<Person>,
    name: &str,
) -> mongodb::error::Result<()> {
    //isim alanında regex (düzenli ifade) kullanarak arama yapar.
    //"i" kısmı ise büyük/küçük harf duyarlılığını kaldırır
    let filter = doc! { "isim": { "$regex": name, "$options": "i" } };
    let cursor = person_collection.find(filter, None).await?;
    //cursor içindeki tüm sonuçları Vec<Person> (Person tipinde bir vektör) içine toplar.
    let results: Vec<Person> = cursor.try_collect().await?;
    if results.is_empty(){
        println!("'{}' Kişisi Bulunamadı.",name)
    }else{
        for person in results {
            println!("{:?}", person);
        }
    }
    Ok(())
}

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Geçersiz giriş");
    input.trim().to_string()
}

