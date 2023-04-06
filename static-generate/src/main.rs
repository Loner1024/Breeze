use std::env::current_dir;
use static_generate::generate::Generator;

fn main() {
    let template_dir = current_dir().unwrap().join("template");
    let target_dir = current_dir().unwrap().join("dist");
    let template_name = "Aragaki";
    let mut gen = Generator::new(template_dir, target_dir, template_name);

    gen.run().unwrap();
}



