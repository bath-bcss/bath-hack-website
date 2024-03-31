use yew::{Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct AccountCVSubpageProps {
    pub cv_exists_handle: UseStateHandle<bool>,
}
