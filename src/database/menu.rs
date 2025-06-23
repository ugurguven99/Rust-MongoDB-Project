use std::error::Error;
use std::io;
use mongodb::bson::Document;
use crate::database::animal_crud::{create_animal, delete_animal, read_animals, update_animal};
use crate::database::person_crud::{create_person, delete_person, read_persons, search_by_name, update_person};
use crate::models::{Animal, Person};
use std::io::{Write}; // flush() metodunu kullanabilmek için bu import gereklidir
use mongodb::{Client, options::ClientOptions}; // Örnek, doğru importları kullanın

pub async fn connect_to_db() -> Result<Client, Box<dyn Error + Send + Sync>> {
    // DB bağlantı kodu
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
pub async fn main_menu() -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = connect_to_db().await?;

    loop {
        println!("\nAna Menü:");
        println!("1- Person işlemleri:");
        println!("2- Animal işlemleri:");
        println!("3- Çıkış");
        print!("Seçiminizi yapın: ");
        io::stdout().flush().unwrap();

        let mut choice_menu = String::new();
        io::stdin().read_line(&mut choice_menu).expect("Geçersiz seçim");
        let choice_menu = choice_menu.trim();

        match choice_menu {
            "1" => person_menu(&client).await?,
            "2" => animal_menu(&client).await?,
            "3" => {
                println!("Çıkış yapıldı.");
                break;
            },
            _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."),
        }
    }
    Ok(())
}

pub async fn person_menu(client: &Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    let database = client.database("Guven");
    let person_collection = database.collection::<Person>("Person");
    let counters_collection = database.collection::<Document>("counters");

    loop {
        println!("\nPerson Menü:");
        println!("1. Yeni Kişi Ekle");
        println!("2. Tüm Kişileri Listele");
        println!("3. Kişiyi Güncelle");
        println!("4. Kişiyi Sil");
        println!("5. İsime Göre Ara");
        println!("6. Ana Menüye Dön");
        println!("7. Çıkış Yap");
        print!("Seçiminizi yapın: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Geçersiz giriş");
        let choice = choice.trim();

        match choice {
            "1" => {
                let person_id = create_person(&person_collection, &counters_collection).await?;
                println!("Yeni kişi başarıyla eklendi, person_id: {}", person_id);
            },
            "2" => read_persons(&person_collection).await?,
            "3" => {
                print!("Güncellenecek kişi ID'sini girin: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Geçersiz giriş");
                let person_id: i64 = id_input.trim().parse().expect("Geçersiz ID");

                update_person(&person_collection, person_id).await?;
            },
            "4" => {
                print!("Silinecek kişi ID'sini girin: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Geçersiz giriş");
                let person_id: i64 = id_input.trim().parse().expect("Geçersiz ID");

                delete_person(&person_collection, person_id).await?;
            },
            "5" => {
                print!("Aranacak isim: ");
                io::stdout().flush().unwrap();
                let mut name_input = String::new();
                io::stdin().read_line(&mut name_input).expect("Geçersiz giriş");
                let name = name_input.trim();

                search_by_name(&person_collection, name).await?;
            },
            "6" => break,
            "7" => {
                println!("Programdan çıkılıyor.");
                break;
            },
            _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."),
        }
    }
    Ok(())
}

pub async fn animal_menu(client: &Client) -> Result<(), Box<dyn Error + Send + Sync>> {
    let database = client.database("Guven");
    let counters_collection = database.collection::<Document>("counters");
    let animal_collection = database.collection::<Animal>("Animal");

    loop {
        println!("\nAnimal Menü:");
        println!("1. Tüm Hayvanları Listele");
        println!("2. Yeni Hayvan Ekle");
        println!("3. Hayvanı Sil");
        println!("4. Hayvanı Güncelle");
        println!("5. Ana Menüye Dön");
        println!("6. Çıkış Yap");
        print!("Seçiminizi yapın: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Geçersiz giriş");
        let choice = choice.trim();

        match choice {
            "1" => read_animals(&animal_collection).await?,
            "2" => {
                let animal_id = create_animal(&animal_collection, &counters_collection).await?;
                println!("Yeni hayvan başarıyla eklendi, animal_id: {}", animal_id);
            },
            "3" => {
                print!("Silinecek hayvanın ID'sini girin: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Geçersiz giriş");
                let animal_id: i64 = id_input.trim().parse().expect("Geçersiz ID");

                delete_animal(&animal_collection, animal_id).await?;
            },
            "4" => {
                print!("Güncellenecek hayvanın ID'sini girin: ");
                io::stdout().flush().unwrap();
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Geçersiz giriş");
                let animal_id: i64 = id_input.trim().parse().expect("Geçersiz ID");

                update_animal(&animal_collection, animal_id).await?;
            },
            "5" => break,
            "6" => {
                println!("Programdan çıkılıyor.");
                break;
            },
            _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."),
        }
    }
    Ok(())
}
