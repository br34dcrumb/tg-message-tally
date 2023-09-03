#![windows_subsystem = "windows"]

extern crate winapi;

use csv::Writer;
use serde_derive::{Deserialize, Serialize};
use serde_json as sj;
use std::{fs::File, io::Read, collections::{HashMap, HashSet}, ptr, os::windows::ffi::OsStrExt, ffi::OsStr};


use winapi::um::winuser::{MessageBoxW, MB_OK, MB_ICONASTERISK};
use winapi::shared::windef::HWND;


#[derive(Serialize, Deserialize)]
struct Channel {
    name: String,
    id: u32,
    messages: Vec<Msg>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<String>,
}

#[derive(Debug)]
struct MyMsg {
    date: String,
    name: String,
}


fn main() -> Result<(), ()> {
    let mut file = File::open("result.json").expect("No file found!");
    let mut buff = String::new();
    file.read_to_string(&mut buff).expect("Error while reading file!");
    
    let data : Channel = sj::from_str(&buff).expect("Error while reading file!");
    let messages = data.messages;

    let mut updates: Vec<MyMsg> = Vec::new();
    
    for i in messages {
        if i.from != None {
            updates.push(MyMsg{date: i.date[0..10].to_string(), name: i.from.unwrap_or_default(),});
        };
    }

    updates.sort_by(|a, b|{
        let date_a = &a.date;
        let date_b = &b.date;
        date_a.cmp(date_b)
    });


    let mut table: HashMap<(String, String), usize> = HashMap::new();

    let mut unique_dates: HashSet<String> = HashSet::new();
    let mut unique_people: HashSet<String> = HashSet::new();

    for msg in &updates {
        let key = (msg.name.clone(), msg.date.clone());
        *table.entry(key).or_insert(0) += 1;
        unique_dates.insert(msg.date.clone());
        unique_people.insert(msg.name.clone());
    }

    let mut unique_dates_sorted: Vec<String> = unique_dates.into_iter().collect();
    unique_dates_sorted.sort();

    let mut unique_people_sorted: Vec<String> = unique_people.into_iter().collect();
    unique_people_sorted.sort();


    let file_path = "result.csv";
    let file = File::create(file_path).expect("Cannot create file 1");

    let mut writer = Writer::from_writer(file);

    let name: String = "Name".to_string();
    let mut header = vec![&name];
    header.extend(unique_dates_sorted.iter());

    writer.write_record(header).expect("Cannot write into file 2");

    for person in unique_people_sorted {
        let mut row = vec![person.clone()];
        for date in &unique_dates_sorted {
            let count = table.get(&(person.clone(), date.clone())).unwrap_or(&0);
            row.push(count.to_string());
        }
        writer.write_record(row).expect("Cannot write into file 3");
    }

    // Final Message

    let title = "Process Completed!";
    let message = "[+] CSV created!";

    let wide_message: Vec<u16> = OsStr::new(message).encode_wide().chain(std::iter::once(0)).collect();
    let title_message: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
    let parent_window: HWND = ptr::null_mut();

    unsafe {
        MessageBoxW(
            parent_window,
            wide_message.as_ptr(),
            title_message.as_ptr(),
            MB_OK | MB_ICONASTERISK,
        );
    }

    Ok(())

}