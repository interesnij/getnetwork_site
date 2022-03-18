use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String) -> UploadedFiles {
        UploadedFiles {
            name: filename.to_string(),
            path: format!("./media/{}", filename.to_string()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Forms {
    pub title: String,
    pub description: String,
    pub content: String,
    pub link: String,
    pub main_image: String,
    pub is_active: bool,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub category_list: Vec<i32>,
    pub tags_list: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CategoriesForm {
    pub name: String,
    pub position: i32,
    pub image: String,
}

pub async fn split_payload(payload: &mut Multipart) -> Forms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: Forms = Forms {
        title: "".to_string(),
        description: "".to_string(),
        content: "".to_string(),
        link: "".to_string(),
        main_image: "".to_string(),
        is_active: true,
        images: Vec::new(),
        videos: Vec::new(),
        category_list: Vec::new(),
        tags_list: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let _list = ["title", "content", "description", "link"]

        if _list.contains(&name){
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "content" {
                        form.content = data_string;
                    } else if field.name() == "description" {
                        form.description = data_string;
                    } else if field.name() == "link" {
                        form.link = data_string;
                    }
                }
            }
        }

        else if name == "category_list[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.category_list.push(_int);
                }
            }
        }

        else if name == "is_active" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        form.is_active = true;
                    } else {
                        form.is_active = false;
                    }
                }
            }
        }

        else if name == "tags_list[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.tags_list.push(_int);
                }
            }
        }

        else if name == "main_image" {
            let xxx: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy: String = xxx.to_string();
            let zzz = yyy + ".jpg";
            let file = UploadedFiles::new(zzz);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("Failed to open hello.txt"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("Failed to open hello.txt");
            }
            files.push(file.clone());
            form.main_image = file.path.clone();
        }

        else if name == "images[]" {
            let xxx: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy: String = xxx.to_string();
            let zzz = yyy + ".jpg";
            let file = UploadedFiles::new(zzz);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("Failed to open hello.txt");
            };
            println!("content_type={:#?}", field.content_type());
            if field.content_type().to_string() == "image/jpeg".to_string() {
                files.push(file.clone());
                form.images.push(file.path.clone());
            };
        }

        else if name == "videos[]" {
            let xxx: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy: String = xxx.to_string();
            let zzz = yyy + ".mp4";
            let file = UploadedFiles::new(zzz);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("Failed to open hello.txt");
            };
            println!("content_type={:#?}", field.content_type());
            if field.content_type().to_string() == "video/mp4".to_string() {
                files.push(file.clone());
                form.videos.push(file.path.clone());
            };
        }
    }
    form
}
pub async fn category_split_payload(payload: &mut Multipart) -> CategoriesForm {
    let mut form: CategoriesForm = CategoriesForm {
        name: "".to_string(),
        position: 0,
        image: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "image" {
            let xxx: i32 = rand::thread_rng().gen_range(0..10000000);
            let yyy: String = xxx.to_string();
            let zzz = yyy + ".jpg";
            let file = UploadedFiles::new(zzz);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("Failed to open hello.txt"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("Failed to open hello.txt");
            }
            form.image = file.path.clone()
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    //let data_string = s.to_string();
                    let _int: i32 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }

        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string
                    } else if field.name() == "position" {
                        form.position = 0
                    }
                }
            }
        }
    }
    form
}
