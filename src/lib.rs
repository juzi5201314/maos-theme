#![feature(box_syntax)]
#![feature(try_blocks)]

use seed::app::{CmdHandle, Orders};
use seed::browser::dom::event_handler::ev;
use seed::virtual_dom::IntoNodes;
use seed::virtual_dom::*;
use seed::{button, div, log, nodes, plain, App, Url, C};
use wasm_bindgen::prelude::wasm_bindgen;

mod fetch;
mod pages;

seed::struct_urls!();

impl<'a> Urls<'a> {
    pub fn index_url(self) -> Url {
        self.base_url()
    }

    pub fn post_url(self, post_id: u32) -> Url {
        self.base_url().add_hash_path_part("post").add_hash_path_part(post_id.to_string())
    }
}

fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_hash_base_url(),
        page: Page::Wait(Page::init(&mut url, orders)),
    }
}

struct Model {
    base_url: Url,
    page: Page,
}

pub enum Msg {
    UrlChanged(seed::prelude::subs::UrlChanged),

    Render(Option<Box<dyn ViewModel>>),
}

enum Page {
    VM(Box<dyn ViewModel>),

    Wait(Option<CmdHandle>),
    NotFound,
}

pub trait ViewModel {
    fn view(&self) -> Vec<Node<Msg>>;
}

impl Page {
    fn init(url: &mut Url, orders: &mut impl Orders<Msg>) -> Option<CmdHandle> {
        Some(match url.next_hash_path_part() {
            None => pages::index::init(url, orders),
            Some("post") => pages::post::init(url, orders),
            _ => {
                orders.send_msg(Msg::Render(None));
                return None;
            }
        })
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(seed::prelude::subs::UrlChanged(mut url)) => {
            model.page = Page::Wait(Page::init(&mut url, orders));
        }
        Msg::Render(Some(vm)) => model.page = Page::VM(vm),
        Msg::Render(None) => model.page = Page::NotFound,
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    nodes![
        header(&model.base_url),
        match &model.page {
            Page::VM(vm) => {
                vm.view()
            }
            Page::NotFound => {
                nodes![plain!("404")]
            }
            Page::Wait(_) => crate::pages::wait::view(),
        }
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    use seed::prelude::IndexMap;
    use seed::virtual_dom::At;
    use seed::{a, attrs, li, ul};
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).index_url() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).post_url(11) },
            "Post",
        ]],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::console_error_panic_hook::set_once();
    App::start("app", init, update, view);
}
