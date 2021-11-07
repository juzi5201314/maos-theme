use seed::virtual_dom::Node;
use crate::Msg;

pub fn view() -> Vec<Node<Msg>> {
    seed::nodes![
        seed::plain!("wait...")
    ]
}
