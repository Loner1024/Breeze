use std::fs::{File, create_dir};
use std::path::PathBuf;
use anyhow::{anyhow, Result};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

pub struct Generator<'reg, 'a> {
    pub template_dir: PathBuf,
    pub target_dir: PathBuf,
    template_registry: Handlebars<'reg>,
    template_name: &'a str,
}

impl<'a> Generator<'_, 'a> {
    pub fn new(template_dir: PathBuf, target_dir: PathBuf, template_name: &'a str) -> Self {
        Self {
            template_dir,
            target_dir,
            template_name,
            template_registry: Handlebars::new(),
        }
    }

    // generate whole site
    pub fn run(&mut self) -> Result<()> {
        // clear target dir
        if self.target_dir.exists() {
            std::fs::remove_dir_all(&self.target_dir)?;
        }

        // create target dir
        create_dir(&self.target_dir)?;
        create_dir(self.target_dir.join("css"))?;

        // copy css file to target
        std::fs::copy(self.template_dir.join(self.template_name).join("css").join("style.css"), self.target_dir.join("css").join("style.css"))?;

        self.template_registry.register_template_file("base", &self.template_dir.join(self.template_name).join("base.hbs"))?;

        // index page
        let index_param = json!(
            {
                "site_title": "Uniix 的博客",
                "parent": "base",
            }
        );


        self.generate("index.hbs", "index.html", &index_param)?;

        Ok(())
    }

    // generate single page
    pub fn generate<T: Serialize>(&mut self, template_file: &'a str, target_file: &str, param: &T) -> Result<PathBuf> {
        let template_file = self.template_dir.join(self.template_name).join(template_file);

        // template file name without extension
        let template_file_name = Option::map(template_file.file_stem(), |name| name.to_str()).flatten().ok_or(anyhow!("template file name without extension"))?;

        // register template
        self.template_registry.register_template_file(template_file_name, &template_file)?;


        let target_file = self.target_dir.join(target_file);

        // creat target file
        let mut target = File::create(target_file.clone())?;

        self.template_registry.render_to_write(template_file_name, param, &mut target)?;

        Ok(target_file)
    }
}
