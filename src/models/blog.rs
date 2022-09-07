use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize,Deserialize};
use crate::models::{User, Tag};
use crate::schema::{
    blog_categories,
    blogs,
    blog_category,
    blog_images,
    blog_videos,
    blog_comments,
};
use crate::utils::establish_connection;


#[derive(Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name="blog_categories"]
pub struct BlogCategories {
    pub id:          i32,
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl BlogCategories {
    pub fn get_blogs_list(&self, page: i32, limit: i32) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = self.get_blogs(limit.into(), step.into());
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
        use crate::schema::{
            blogs::dsl::blogs,
            blog_category::dsl::blog_category,
        };

        let _connection = establish_connection();
        let ids = blog_category
            .filter(schema::blog_category::blog_categories_id.eq(self.id))
            .select(schema::blog_category::blog_id)
            .load::<i32>(&_connection)
            .expect("E");
        return blogs
            .filter(schema::blogs::id.eq_any(ids))
            .filter(schema::blogs::is_active.eq(true))
            .order(schema::blogs::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Blog>(&_connection)
            .expect("E.");
    }
    pub fn get_6_blogs(&self) -> Vec<Blog> {
        use crate::schema::{
            blogs::dsl::blogs,
            blog_category::dsl::blog_category,
        };

        let _connection = establish_connection();
        let ids = blog_category
            .filter(schema::blog_category::blog_categories_id.eq(self.id))
            .select(schema::blog_category::blog_id)
            .load::<i32>(&_connection)
            .expect("E");
        return blogs
            .filter(schema::blogs::id.eq_any(ids))
            .filter(schema::blogs::is_active.eq(true))
            .order(schema::blogs::created.desc())
            .limit(6)
            .load::<Blog>(&_connection)
            .expect("E.");
    }

    pub fn get_blogs_ids(&self) -> Vec<i32> {
        use crate::schema::blog_category::dsl::blog_category;

        let _connection = establish_connection();
        return blog_category
            .filter(schema::blog_category::blog_categories_id.eq(self.id))
            .select(schema::blog_category::blog_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_all_blogs(&self) -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        return blogs
            .filter(schema::blogs::id.eq_any(self.get_blogs_ids()))
            .order(schema::blogs::created.desc())
            .load::<Blog>(&_connection)
            .expect("E");
    }

    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
}

#[derive(Insertable)]
#[table_name="blog_categories"]
pub struct NewBlogCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
    pub count:       i16,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="blog_categories"]
pub struct EditBlogCategories {
    pub name:        String,
    pub description: Option<String>,
    pub position:    i16,
    pub image:       Option<String>,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable, Associations)]
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
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl Blog {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_100_description(&self) -> String {
        if self.description.is_some() {
            let _content = self.description.as_deref().unwrap();
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

    pub fn get_categories(&self) -> Vec<BlogCategories> {
        use crate::schema::{
            blog_category::dsl::blog_category,
            blog_categories::dsl::blog_categories,
        };

        let _connection = establish_connection();
        let ids = blog_category
            .filter(schema::blog_category::blog_id.eq(self.id))
            .select(schema::blog_category::blog_categories_id)
            .load::<i32>(&_connection)
            .expect("E");

        return blog_categories
            .filter(schema::blog_categories::id.eq_any(ids))
            .load::<BlogCategories>(&_connection)
            .expect("E");
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        use crate::schema::tags_items::dsl::tags_items;
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::blog_id.eq(&self.id))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        schema::tags::table
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E")
    }

    pub fn get_3_publish_blogs() -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        return blogs
            .filter(schema::blogs::is_active.eq(true))
            .order(schema::blogs::created.desc())
            .limit(6)
            .load::<Blog>(&_connection)
            .expect("E.");
    }
    pub fn get_3_blogs(user: &User) -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        if user.is_superuser() {
            return blogs
                .order(schema::blogs::created.desc())
                .limit(6)
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return blogs
                .filter(schema::blogs::is_active.eq(true))
                .order(schema::blogs::created.desc())
                .limit(6)
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_blogs_list_for_ids(user: &User, page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Blog::get_blogs_for_ids(user, limit.into(), step.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Blog::get_blogs_for_ids(user, limit.into(), 0, &ids);
        }
        if Blog::get_blogs_for_ids(user, 1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }
    pub fn get_publish_blogs_list_for_ids(page: i32, limit: i32, ids: &Vec<i32>) -> (Vec<Blog>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Blog::get_publish_blogs_for_ids(limit.into(), step.into(), &ids);
        }
        else {
            have_next = limit + 1;
            object_list = Blog::get_publish_blogs_for_ids(limit.into(), 0, &ids);
        }
        if Blog::get_publish_blogs_for_ids(1, have_next.into(), &ids).len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }
    pub fn get_publish_blogs_for_ids(limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        return blogs
            .filter(schema::blogs::id.eq_any(ids))
            .filter(schema::blogs::is_active.eq(true))
            .order(schema::blogs::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Blog>(&_connection)
            .expect("E.");
    }
    pub fn get_blogs_for_ids(user: &User, limit: i64, offset: i64, ids: &Vec<i32>) -> Vec<Blog> {
        use crate::schema::blogs::dsl::blogs;

        let _connection = establish_connection();
        if user.is_superuser() {
            return blogs
                .filter(schema::blogs::id.eq_any(ids))
                .order(schema::blogs::created.desc())
                .limit(limit)
                .offset(offset)
                .load::<Blog>(&_connection)
                .expect("E.");
        }
        else {
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
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
}

impl NewBlog {
    pub fn create (
        title: String,
        description: String,
        link: String,
        image: String,
        is_active: bool,
        user_id: i32) -> Self {
        use chrono::Duration;

        NewBlog {
            title:       title,
            description: Some(description),
            link:        Some(link),
            image:       Some(image),
            is_active:   is_active,
            user_id:     user_id,
            created:     chrono::Local::now().naive_utc() + Duration::hours(3),
            view:        0,
            height:      0.0,
            seconds:     0,
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
    pub fn create (
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
    pub fn create (
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
