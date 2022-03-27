pub mod work_progs;
pub mod blog_progs;
pub mod service_progs;
pub mod serve_progs;
pub mod store_progs;
pub mod wiki_progs;
pub mod pages;
pub mod tag_progs;
pub mod search_progs;

pub use self::{
    work_progs::*,
    blog_progs::*,
    service_progs::*,
    serve_progs::*,
    store_progs::*,
    wiki_progs::*,
    pages::*,
    search_progs::*,
};
