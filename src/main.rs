use std::error::Error;
use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Select};

use config::Config;
use templates::{generate_tsconfig, TEMPLATE_NAMES};

mod config;
mod templates;

fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
	let theme = ColorfulTheme {
		values_style: Style::new().yellow().dim(),
		..ColorfulTheme::default()
	};

	let ascii_art = r#"
			$$$$$$$$\ $$$$$$\          $$\                                   $$\          $$\                       
			\__$$  __$$  __$$\         $$ |                                  $$ |         $$ |                      
				$$ |  $$ /  \__|      $$$$$$\   $$$$$$\ $$$$$$\$$$$\  $$$$$$\ $$ |$$$$$$\$$$$$$\   $$$$$$\  $$$$$$$\ 
				$$ |  \$$$$$$\        \_$$  _| $$  __$$\$$  _$$  _$$\$$  __$$\$$ |\____$$\_$$  _| $$  __$$\$$  _____|
				$$ |   \____$$\         $$ |   $$$$$$$$ $$ / $$ / $$ $$ /  $$ $$ |$$$$$$$ |$$ |   $$$$$$$$ \$$$$$$\  
				$$ |  $$\   $$ |        $$ |$$\$$   ____$$ | $$ | $$ $$ |  $$ $$ $$  __$$ |$$ |$$\$$   ____|\____$$\ 
				$$ |  \$$$$$$  |        \$$$$  \$$$$$$$\$$ | $$ | $$ $$$$$$$  $$ \$$$$$$$ |\$$$$  \$$$$$$$\$$$$$$$  |
				\__|   \______/          \____/ \_______\__| \__| \__$$  ____/\__|\_______| \____/ \_______\_______/ 
																														$$ |                                            
																														\__|                                            
	"#;

	println!("{}", style(ascii_art).blue());

	let template = Select::with_theme(&theme)
		.with_prompt("Choose a template")
		.default(0)
		.items(&TEMPLATE_NAMES)
		.interact()?;

	generate_tsconfig(template)?;

	Ok(Some(Config { template }))
}

fn main() {
	match init_config() {
		Ok(None) => println!("Aborted."),
		Ok(Some(_)) => println!("Generated {} file in root path", style("tsconfig.json").blue()),
		Err(err) => println!("error: {}", err),
	}
}
