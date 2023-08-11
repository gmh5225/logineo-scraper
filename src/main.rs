use std::{fs::OpenOptions, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PersonalInfo {
    #[serde(rename = "hasPhoto")]
    has_photo: i8,
    role: String,
    #[serde(rename = "c_cn")]
    real_name: String,
    #[serde(rename = "containerName")]
    container_name: String,
    c_component: String,
    id: String,
    c_uid: String,
    c_screenname: String,
    c_sn: String,
    c_telephonenumber: String,
    c_name: String,
    c_givenname: String,
    // im unsure what this field is supposed to be
    #[serde(rename = "fn")]
    function_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    contacts: Vec<PersonalInfo>,
    #[serde(rename = "searchText")]
    search_text: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("output.json")
        .unwrap();

    // read file into a string
    let mut file_content: Content = serde_json::from_reader(file).unwrap();
    for i in b'A'..=b'Z' {
        for j in b'A'..=b'Z' {
            for z in b'A'..=b'Z' {
                let combination = format!("{}{}{}", i as char, j as char, z as char);
                println!("Getting contacts starting with {}", combination);
                let res = client.get(
            format!("", combination)
        ).header("Cookie", "").send().await.unwrap();

                if res.status() != 200 {
                    println!("Error: {}", res.status());
                    return;
                }
                let content: Content = res.json().await.unwrap();

                content.contacts.iter().for_each(|info| {
                    if file_content
                        .contacts
                        .iter()
                        .find(|c| c.id == info.id)
                        .is_none()
                    {
                        file_content.contacts.push(info.clone());
                    } else {
                        println!("{} already exists, skipping", info.c_name)
                    }
                });

                tokio::time::sleep(Duration::from_millis(250)).await;
            }
        }
    }

    file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("output.json")
        .unwrap();

    serde_json::to_writer_pretty(file, &file_content).unwrap();
}
