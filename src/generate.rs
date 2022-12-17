use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{bail, Result};

use crate::blogposts::blog_posts;

pub fn generate(out_path: PathBuf) -> Result<()> {
    println!("blog posts: {:?}", blog_posts());
    if !out_path.exists() {
        bail!("Output path {:?} doesn't exist", out_path);
    }
    for post in blog_posts() {
        let mut file = File::create(out_path.join(format!("{}.html", post.slug())))?;
        write!(
            file,
            "<link rel=\"stylesheet\" href=\"https://piperswe.me/assets/pink.css\" />\n{}",
            post.render()
        )?;
    }
    Ok(())
}
