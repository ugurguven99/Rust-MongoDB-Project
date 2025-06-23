use mongodb::Collection;
use crate::models::{Animal};
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use std::io::{Write};

pub async fn read_animals(animals_collection: &Collection<Animal>) -> mongodb::error::Result<()> {
    let cursor = animals_collection.find(None, None).await?;
    let results: Vec<Animal> = cursor.try_collect().await?;
    for animal in results {
        println!("{:?}", animal);
    }
    Ok(())
}

pub async fn create_animal(
    animal_collection: &Collection<Animal>,
    counters_collection: &Collection<Document>,
) -> mongodb::error::Result<i64> {
    let before_update = counters_collection.find_one(doc! { "_id": "animal_id" }, None).await?;
    println!("Sayaç güncellemesinden önceki değer: {:?}", before_update);

    let update_options = mongodb::options::UpdateOptions::builder().upsert(true).build();
    counters_collection
        .update_one(doc! { "_id": "animal_id" }, doc! { "$inc": { "sek": 1 } }, update_options)
        .await?;

    let after_update = counters_collection.find_one(doc! { "_id": "animal_id" }, None).await?;
    println!("Sayaç güncellemesinden sonraki değer: {:?}", after_update);

    let next_animal_id = match after_update {
        Some(doc) => match doc.get_i64("sek") {
            Ok(sek) => sek,
            Err(_) => doc.get_i32("sek").unwrap_or(0) as i64,
        },
        None => 0,
    };

    println!("Sonraki animal_id: {}", next_animal_id);

    let cins = read_input("Cins: ");
    let cinsiyet = read_input("Cinsiyet: ");
    let ayak_sayisi: i32 = read_input("Ayak: ").trim().parse().expect("Geçersiz ayak");

    let new_animal = Animal {
        animal_id: next_animal_id,
        cins,
        cinsiyet,
        ayak_sayisi,

    };

    animal_collection.insert_one(new_animal, None).await?;
    println!("Yeni hayvan başarıyla eklendi");

    Ok(next_animal_id)
}
pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Geçersiz giriş");
    input.trim().to_string()
}
pub async fn delete_animal(
    animal_collection: &Collection<Animal>,
    animal_id: i64,
) -> mongodb::error::Result<()> {
    let filter = doc! { "animal_id": animal_id };
    let delete_result = animal_collection.delete_one(filter, None).await?;
    println!("Silinen belge sayısı: {}", delete_result.deleted_count);
    Ok(())
}
pub async fn update_animal(
    animal_collection: &Collection<Animal>,
    animal_id: i64,
) -> mongodb::error::Result<()> {
    let filter = doc! { "animal_id": animal_id };

    let yeni_cins = read_input("Yeni cins (boş bırakırsanız değişmeyecek): ");
    let yeni_cinsiyet = read_input("Yeni cinsiyet (boş bırakırsanız değişmeyecek): ");
    let yeni_ayak_sayisi = read_input("Yeni ayak sayısı (boş bırakırsanız değişmeyecek): ");

    let mut update_doc = doc! {};

    if !yeni_cins.trim().is_empty() {
        update_doc.insert("cins", yeni_cins);
    }
    if !yeni_cinsiyet.trim().is_empty() {
        update_doc.insert("cinsiyet", yeni_cinsiyet);
    }
    if let Ok(ayak_sayisi) = yeni_ayak_sayisi.trim().parse::<i32>() {
        update_doc.insert("ayak_sayisi", ayak_sayisi);
    }

    let update = doc! { "$set": update_doc };

    let update_result = animal_collection.update_one(filter, update, None).await?;
    if update_result.matched_count > 0 {
        println!("animal_id {} başarıyla güncellendi.", animal_id);
    } else {
        println!("animal_id {} için hiçbir belge bulunamadı.", animal_id);
    }

    let updated_animal = animal_collection.find_one(doc! { "animal_id": animal_id }, None).await?;
    match updated_animal {
        Some(animal) => println!("Güncellenmiş hayvan: {:?}", animal),
        None => println!("Hayvan bulunamadı"),
    }
    Ok(())
}