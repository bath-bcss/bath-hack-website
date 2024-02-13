pub fn build_path(path: String) -> String {
    let base_url = std::env!("BHW_FRONTEND_API_URL").to_string();
    base_url + path.as_str()
}
