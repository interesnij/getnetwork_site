use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    BelongingToDsl,
    QueryDsl,
};
use serde::{Serialize,Deserialize};
use crate::models::User;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="blog_categories"]
pub struct BlogCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

impl BlogCategories {
    pub fn get_blogs_list(&self, page: i32, limit: i32) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            have_next = page * limit + 1;
            object_list = self.get_blogs(limit.into(), have_next.into());
        }
        else {
            have_next = limit + 1;
            object_list = self.get_blogs(limit.into(), 0);
        }
        if self.get_blogs(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_blogs(&self, limit: i64, offset: i64) -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        let ids = BlogCategory::belonging_to(self)
            .select(schema::blog_category::blog_id);
        return blogs
            .filter(schema::blogs::id.eq_any(ids))
            .filter(schema::blogs::is_active.eq(true))
            .order(schema::blogs::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Blog>(&_connection)
            .expect("E.");
    }
}

#[derive(Insertable)]
#[table_name="blog_categories"]
pub struct NewBlogCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="blog_categories"]
pub struct EditBlogCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i32,
    pub image:       Option<String>,
    pub count:       i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Blog {
    pub id:          i32,
    pub title:       String,
    pub description: Option<String>,
    pub content:     Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl Blog {
    pub fn get_100_description(&self) -> String {
        if self.content.is_some() {
            let _content = self.content.as_deref().unwrap();
            if _content.len() > 100 {
                return _content[..100].to_string();
            }
            else {
                return _content.to_string();
            }
        }
        else {
            return "".to_string();
        }
    }
}

#[derive(Serialize, Insertable)]
#[table_name="blogs"]
pub struct NewBlog {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub user_id:     i32,
    pub created:     chrono::NaiveDateTime,
}

impl NewBlog {
    pub fn from_blog_form(
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        user_id: i32) -> Self {
        NewBlog {
            title:       title,
            description: Some(description),
            link:        Some(link),
            image:       Some(image),
            is_active:   is_active,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc(),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="blogs"]
pub struct EditBlog {
    pub title:       String,
    pub description: Option<String>,
    pub link:        Option<String>,
    pub image:       Option<String>,
    pub is_active:   bool,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(BlogCategories)]
#[belongs_to(Blog)]
#[table_name="blog_category"]
pub struct BlogCategory {
    pub id:                 i32,
    pub blog_categories_id: i32,
    pub blog_id:            i32,
}

#[derive(Insertable)]
#[table_name="blog_category"]
pub struct NewBlogCategory {
    pub blog_categories_id: i32,
    pub blog_id:            i32,
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Blog, foreign_key="blog")]
pub struct BlogImage {
    pub id:   i32,
    pub blog: i32,
    pub src:  String
}

#[derive(Serialize, Insertable)]
#[table_name="blog_images"]
pub struct NewBlogImage {
    pub blog: i32,
    pub src:  String
}

impl NewBlogImage {
    pub fn from_blog_images_form (
        blog_id: i32, src: String) -> Self {
        NewBlogImage {
            blog: blog_id,
            src:  src
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Blog, foreign_key="blog")]
pub struct BlogVideo {
    pub id:   i32,
    pub blog: i32,
    pub src:  String
}

#[derive(Serialize, Insertable)]
#[table_name="blog_videos"]
pub struct NewBlogVideo {
    pub blog: i32,
    pub src:  String
}

impl NewBlogVideo {
    pub fn from_blog_videos_form (
        blog_id: i32, src: String) -> Self {
        NewBlogVideo {
            blog: blog_id,
            src:  src
        }
    }
}

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Blog)]
#[belongs_to(User)]
pub struct BlogComment {
    pub id:        i32,
    pub comment:   String,
    pub blog_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="blog_comments"]
pub struct NewBlogComment {
    pub comment:   String,
    pub blog_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

impl NewBlogComment {
    pub fn new(comment: String, blog_id: i32,
        user_id: i32, parent_id: Option<i32>) -> Self{
        NewBlogComment {
            comment:   comment,
            blog_id:   blog_id,
            user_id:   user_id,
            parent_id: parent_id,
            created:   chrono::Local::now().naive_utc(),
        }
    }
}
