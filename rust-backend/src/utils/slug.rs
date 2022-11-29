use rand::Rng;

pub fn generate_slug(title: &str) -> String {
    let slug = {
        let post_slug = title.to_lowercase().replace(' ', "-");
        let re = regex::Regex::new(r"[^a-z0-9-]").unwrap();
        let post_slug = re.replace_all(&post_slug, "").to_string();

        // add random number to end of slug
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(1000..9999);
        format!("{}-{}", post_slug, random_number)
    };

    slug
}
