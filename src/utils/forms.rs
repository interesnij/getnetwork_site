use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};

#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String,
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String, owner_id: i32) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/{}/{}/{}/{}/",
            owner_id.to_string(),
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        let create_path = format_folder.replace("./", "/my/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Forms {
    pub title:         String,
    pub description:   String,
    pub link:          String,
    pub main_image:    String,
    pub is_active:     bool,
    pub images:        Vec<String>,
    pub videos:        Vec<String>,
    pub category_list: Vec<i32>,
    pub tags_list:     Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedbackForm {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CategoriesForm {
    pub name:        String,
    pub description: String,
    pub position:    i16,
    pub image:       String,
    pub level:       i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeCategoriesForm {
    pub name:            String,
    pub description:     String,
    pub tech_categories: i32,
    pub position:        i16,
    pub default_price:   i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ContentForm {
    pub content: Option<String>,
}

pub async fn item_form(payload: &mut Multipart, owner_id: i32) -> Forms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: Forms = Forms {
        title:         "".to_string(),
        description:   "".to_string(),
        link:          "".to_string(),
        main_image:    "".to_string(),
        is_active:     true,
        images:        Vec::new(),
        videos:        Vec::new(),
        category_list: Vec::new(),
        tags_list:     Vec::new(),
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
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
        }

        else if name == "images[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                files.push(file.clone());
                form.images.push(file.path.clone().replace("./","/"));
            }
        }

        else if name == "videos[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                files.push(file.clone());
                form.videos.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}
pub async fn category_form(payload: &mut Multipart, owner_id: i32) -> CategoriesForm {
    let mut form: CategoriesForm = CategoriesForm {
        name:        "".to_string(),
        description: "".to_string(),
        position:    0,
        image:       "".to_string(),
        level:       0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                form.image = file.path.clone().replace("./","/");
            }
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "level" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.level = _int;
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

pub async fn content_form(payload: &mut Multipart) -> ContentForm {
    let mut form: ContentForm = ContentForm {
        content: None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "content" {
                    form.content = Some(data_string)
                }
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct StoreForms {
    pub title:          String,
    pub description:    String,
    pub link:           String,
    pub main_image:     String,
    pub is_active:      bool,
    pub images:         Vec<String>,
    pub videos:         Vec<String>,
    pub category_list:  Vec<i32>,
    pub tags_list:      Vec<i32>,
    pub serve_list:     Vec<i32>,
    pub close_tech_cats_list: Vec<i32>,
    pub position:       i16,
}

// форма для элементов с опциями / тех категориями
pub async fn store_form(payload: &mut Multipart, owner_id: i32) -> StoreForms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: StoreForms = StoreForms {
        title:                "".to_string(),
        description:          "".to_string(),
        link:                 "".to_string(),
        main_image:           "".to_string(),
        is_active:            true,
        images:               Vec::new(),
        videos:               Vec::new(),
        category_list:        Vec::new(),
        tags_list:            Vec::new(),
        serve_list:           Vec::new(),
        close_tech_cats_list: Vec::new(),
        position:             0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "description", "link"];

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
        else if name == "close_tech_cats_list[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.close_tech_cats_list.push(_int);
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
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
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
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
        }

        else if name == "images[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                files.push(file.clone());
                form.images.push(file.path.clone().replace("./","/"));
            }
        }

        else if name == "videos[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                files.push(file.clone());
                form.videos.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

pub async fn serve_category_form(payload: &mut Multipart, _owner_id: i32) -> ServeCategoriesForm {
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
                    let _int: i16 = s.parse().unwrap();
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


#[derive(Deserialize, Serialize, Debug)]
pub struct OrderForms {
    pub title:       String,
    pub types:       i16,
    pub object_id:   i32,
    pub username:    String,
    pub description: Option<String>,
    pub email:       String,
    pub files:       Vec<String>,
    pub serve_list:  Vec<String>,
}

// форма для заказов
pub async fn order_form(payload: &mut Multipart, owner_id: i32) -> OrderForms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: OrderForms = OrderForms {
        title:       "".to_string(),
        types:       0,
        object_id:   0,
        username:    "".to_string(),
        description: None,
        email:       "".to_string(),
        files:       Vec::new(),
        serve_list:  Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "email", "description", "username"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "email" {
                        form.email = data_string;
                    } else if field.name() == "username" {
                        form.username = data_string;
                    }
                }
            }
        }
        else if name == "object_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.object_id = _int;
                }
            }
        }
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }
        else if name == "serve_list" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let v: Vec<&str> = data_string.split(",").collect();
                    for i in v {
                        let _int: i32 = i.parse().unwrap();
                        form.serve_list.push(_int);
                    }
                }
            }
        }
        else if name == "close_tech_cats_list[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    let _int: i32 = data_string.parse().unwrap();
                    form.close_tech_cats_list.push(_int);
                }
            }
        }
        else if name == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string(), owner_id);
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
                files.push(file.clone());
                form.files.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}
