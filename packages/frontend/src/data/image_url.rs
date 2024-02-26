pub fn get_image_url(image_name: String) -> String {
    let mut image_name = image_name;
    image_name.insert_str(0, "/img/");
    image_name
}
