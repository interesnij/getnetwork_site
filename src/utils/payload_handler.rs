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
    pub link: String,
    pub main_image: String,
    pub is_active: bool,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub category_list: Vec<i32>,
    pub tags_list: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StoreForms {
    pub title: String,
    pub description: String,
    pub link: String,
    pub main_image: String,
    pub is_active: bool,
    pub price: i32,
    pub price_acc: i32,
    pub social_price: i32,
    pub images: Vec<String>,
    pub videos: Vec<String>,
    pub category_list: Vec<i32>,
    pub tags_list: Vec<i32>,
    pub serve_list: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedbackForm {
    pub username: String,
    pub email: String,
    pub message: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CategoriesForm {
    pub name: String,
    pub description: String,
    pub position: i32,
    pub image: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct ServeCategoriesForm {
    pub name: String,
    pub description: String,
    pub tech_categories: i32,
    pub position: i32,
    pub default_price: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentForm {
    pub content: String,
}

pub async fn item_form(payload: &mut Multipart) -> Forms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: Forms = Forms {
        title: "".to_string(),
        description: "".to_string(),
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
        let _list = ["title", "description", "link"];

        if _list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
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
            let xxx1: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy1: String = xxx1.to_string();
            let zzz1 = yyy1 + ".jpg";
            let file = UploadedFiles::new(zzz1);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            }
            form.main_image = file.path.clone().replace("./","/");
        }

        else if name == "images[]" {
            let xxx2: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy2: String = xxx2.to_string();
            let zzz2 = yyy2 + ".jpg";
            let file = UploadedFiles::new(zzz2);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            if field.content_type().to_string() == "image/jpeg".to_string() {
                files.push(file.clone());
                form.images.push(file.path.clone().replace("./","/"));
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
                    .expect("E");
            };
            if field.content_type().to_string() == "video/mp4".to_string() {
                files.push(file.clone());
                form.videos.push(file.path.clone().replace("./","/"));
            };
        }
    }
    form
}
pub async fn category_form(payload: &mut Multipart) -> CategoriesForm {
    let mut form: CategoriesForm = CategoriesForm {
        name: "".to_string(),
        description: "".to_string(),
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
            form.image = file.path.clone().replace("./","/")
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
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
                    } else if field.name() == "description" {
                        form.description = data_string
                    }
                }
            }
        }
    }
    form
}

pub async fn store_form(payload: &mut Multipart) -> StoreForms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: StoreForms = StoreForms {
        title: "".to_string(),
        description: "".to_string(),
        link: "".to_string(),
        main_image: "".to_string(),
        is_active: true,
        price: 0,
        price_acc: 0,
        social_price: 0,
        images: Vec::new(),
        videos: Vec::new(),
        category_list: Vec::new(),
        tags_list: Vec::new(),
        serve_list: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "description", "link"];
        let i32_list = ["price", "price_acc", "social_price"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "description" {
                        form.description = data_string;
                    } else if field.name() == "link" {
                        form.link = data_string;
                    }

                }
            }
        }
        else if i32_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    if field.name() == "price" {
                        form.price = _int;
                    } else if field.name() == "price_acc" {
                        form.price_acc = _int;
                    } else if field.name() == "social_price" {
                        form.social_price = _int;
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

        else if name == "serve_list[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.serve_list.push(_int);
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
            let xxx1: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy1: String = xxx1.to_string();
            let zzz1 = yyy1 + ".jpg";
            let file = UploadedFiles::new(zzz1);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            }
            form.main_image = file.path.clone().replace("./","/");
        }

        else if name == "images[]" {
            let xxx2: i32 = rand::thread_rng().gen_range(0..100000);
            let yyy2: String = xxx2.to_string();
            let zzz2 = yyy2 + ".jpg";
            let file = UploadedFiles::new(zzz2);
            let file_path = file.path.clone();
            let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await
                    .unwrap()
                    .expect("E");
            };
            if field.content_type().to_string() == "image/jpeg".to_string() {
                files.push(file.clone());
                form.images.push(file.path.clone().replace("./","/"));
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
                    .expect("E");
            };
            if field.content_type().to_string() == "video/mp4".to_string() {
                files.push(file.clone());
                form.videos.push(file.path.clone().replace("./","/"));
            };
        }
    }
    form
}

pub async fn serve_category_form(payload: &mut Multipart) -> ServeCategoriesForm {
    let mut form: ServeCategoriesForm = ServeCategoriesForm {
        name: "".to_string(),
        description: "".to_string(),
        tech_categories: 0,
        position: 0,
        default_price: 0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "tech_categories" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.tech_categories = _int;
                }
            }
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "default_price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.default_price = _int;
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
                    } else if field.name() == "description" {
                        form.description = data_string
                    }
                }
            }
        }
    }
    form
}

pub async fn feedback_form(payload: &mut Multipart) -> FeedbackForm {
    let mut form: FeedbackForm = FeedbackForm {
        username: "".to_string(),
        email: "".to_string(),
        message: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "email" {
                    form.email = data_string
                } else if field.name() == "message" {
                    form.message = data_string
                }
            }
        }
    }
    form
}
