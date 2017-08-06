use liquid;
use liquid::LiquidOptions;
use liquid::Template;
use liquid::Error;

fn load_build_template() -> Result<Template, Error> {
  liquid::parse(include_str!("templates/BUILD.template"),
                LiquidOptions::default())
}
