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
        // вариант для https
        let create_path = format_folder.replace("./", "/my/");
        // вариант для debug
        //let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedbackForm {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CategoriesForm {
    pub name:           String,
    pub name_en:        String,
    pub description:    String,
    pub description_en: String,
    pub position:       i16,
    pub image:          String,
    pub level:          i16,
    pub types:          i16,
    pub slug:           String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ContentForm {
    pub content:    Option<String>,
    pub content_en: Option<String>,
}

pub async fn category_form(payload: &mut Multipart, owner_id: i32) -> CategoriesForm {
    let mut form: CategoriesForm = CategoriesForm {
        name:        "".to_string(),
        description: "".to_string(),
        position:    0,
        image:       "".to_string(),
        level:       0,
        types:       0,
        slug:        "".to_string(),
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
        else if name == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
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
                    } else if field.name() == "slug" {
                        form.slug = data_string
                    }
                }
            }
        }
    }
    form
}

pub async fn content_form(payload: &mut Multipart) -> ContentForm {
    let mut form: ContentForm = ContentForm {
        content:    None,
        content_en: None,
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
                else if field.name() == "content_en" {
                    form.content_en = Some(data_string)
                }
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ItemForms {
    pub title:          String,
    pub title_en:       String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub link:           Option<String>,
    pub main_image:     Option<String>,
    pub category_list:  Vec<i32>,
    pub tags_list:      Vec<i32>,
    pub serve_list:     Vec<i32>,
    pub close_tech_cats_list: Vec<i32>,
    pub position:       i16,
    pub types:          i16,
    pub slug:           String,
}

// форма для элементов с опциями / тех категориями
pub async fn item_form(payload: &mut Multipart, owner_id: i32) -> ItemForms {
    let mut form: ItemForms = ItemForms {
        title:          "".to_string(),
        title_en:       "".to_string(),
        description:    None,
        description_en: None,
        link:           None,
        main_image:     None,
        category_list:  Vec::new(),
        tags_list:      Vec::new(),
        serve_list:     Vec::new(),
        close_tech_cats_list: Vec::new(),
        position:       0,
        types:          0,
        slug:           "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "title_en", "description", "description_en", "link", "slug"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "title_en" {
                        form.title_en = data_string; 
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "description_en" {
                        form.description_en = Some(data_string);
                    } else if field.name() == "link" {
                        form.link = Some(data_string);
                    }
                    else if field.name() == "slug" {
                        form.slug = data_string;
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
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
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
                form.main_image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

pub async fn feedback_form(payload: &mut Multipart) -> FeedbackForm {
    let mut form: FeedbackForm = FeedbackForm {
        username: "".to_string(),
        email:    "".to_string(),
        message:  "".to_string(),
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
    pub title:          String,
    pub title_en:       String,
    pub types:          i16,
    pub object_id:      i32,
    pub username:       String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub email:          String,
    pub files:          Vec<String>,
    pub serve_list:     Vec<i32>,
}

// форма для заказов
pub async fn order_form(payload: &mut Multipart, owner_id: i32) -> OrderForms {
    let mut files: Vec<UploadedFiles> = Vec::new();

    let mut form: OrderForms = OrderForms {
        title:          "".to_string(),
        title_en:       "".to_string(),
        types:          0,
        object_id:      0,
        username:       "".to_string(),
        description:    None,
        description_en: None,
        email:          "".to_string(),
        files:          Vec::new(),
        serve_list:     Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "title_en", "description", "description_en", "email", "username"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "title_en" {
                        form.title_en = data_string;
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "description_en" {
                        form.description_en = Some(data_string);
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
                    for i in v.iter() {
                        let _int: i32 = i.parse().unwrap();
                        form.serve_list.push(_int);
                    }
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

#[derive(Deserialize, Serialize, Debug)]
pub struct FileForm {
    pub item_types: i16,      // блог, услуга ......
    pub types:      i16,      // фото, видео, аудио ......
    pub files:      Vec<String>,
}
pub async fn files_form(payload: &mut Multipart, owner_id: i32) -> FileForm {
    let mut _files: Vec<UploadedFiles> = Vec::new();

    let mut form: FileForm = FileForm {
        item_types: 0,
        types:      0,
        files:      Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "files[]" {
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
                _files.push(file.clone());
                form.files.push(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "item_types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.item_types = _int;
                }
            }
        }
        else if field.name() == "types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.types = _int;
                }
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeCategoriesForm {
    pub name:          String,
    pub description:   String,
    pub category_id:   i32,
    pub position:      i16,
    pub default_price: i32,
}
pub async fn serve_category_form(payload: &mut Multipart, _owner_id: i32) -> ServeCategoriesForm {
    let mut form: ServeCategoriesForm = ServeCategoriesForm {
        name:          "".to_string(),
        description:   "".to_string(),
        category_id:   0,
        position:      0,
        default_price: 0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "category_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.category_id = _int;
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
