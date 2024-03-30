use std::io::Write;
use std::{error::Error, fs::File};
use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)]
#[allow(dead_code)]
struct Config {
    template: usize,
}

fn generate_tsconfig(template: usize) -> std::io::Result<()> {
    let mut file = File::create("tsconfig.json")?;
    
    match template {
        0 => { //Recommended
            let content = "{ 
  \"compilerOptions\": {
    \"target\": \"es2016\",
    \"module\": \"commonjs\",
    \"esModuleInterop\": true,
    \"forceConsistentCasingInFileNames\": true,
    \"strict\": true,
    \"skipLibCheck\": true
  },
  \"$schema\": \"https://json.schemastore.org/tsconfig\"
}";
            file.write_all(content.as_bytes())?;
        },
        1 => { //Node 21
            let content = "{
  \"$schema\": \"https://json.schemastore.org/tsconfig\",
  \"_version\": \"21.0.0\",
            
  \"compilerOptions\": {
    \"lib\": [\"es2023\"],
    \"module\": \"node16\",
    \"target\": \"es2022\",
              
    \"strict\": true,
    \"esModuleInterop\": true,
    \"skipLibCheck\": true,
    \"moduleResolution\": \"node16\"
  }
}";
            file.write_all(content.as_bytes())?;
        },
        2 => {//Node 20
            let content = "{
  \"$schema\": \"https://json.schemastore.org/tsconfig\",
  \"_version\": \"20.1.0\",
              
  \"compilerOptions\": {
    \"lib\": [\"es2023\"],
    \"module\": \"node16\",
    \"target\": \"es2022\",
             
    \"strict\": true,
    \"esModuleInterop\": true,
    \"skipLibCheck\": true,
    \"moduleResolution\": \"node16\"
}
}";
            file.write_all(content.as_bytes())?;
        },
        3 => {//Node 19
            let content = "{
  \"$schema\": \"https://json.schemastore.org/tsconfig\",
              
  \"_version\": \"19.1.0\",
              
  \"compilerOptions\": {
    \"lib\": [\"es2023\"],
    \"module\": \"node16\",
    \"target\": \"es2022\",
              
    \"strict\": true,
    \"esModuleInterop\": true,
    \"skipLibCheck\": true,
    \"moduleResolution\": \"node16\"
    }
}";
            file.write_all(content.as_bytes())?;
        },
        4 => {//Bun
            let content = "{
  // This is based on https://bun.sh/docs/typescript#suggested-compileroptions
  \"$schema\": \"https://json.schemastore.org/tsconfig\",
  \"docs\": \"https://bun.sh/docs/typescript\",

  \"compilerOptions\": {
    // Enable latest features
    \"lib\": [\"ESNext\"],
    \"target\": \"ESNext\",
    \"module\": \"ESNext\",
    \"moduleDetection\": \"force\",
    \"jsx\": \"react-jsx\",
    \"allowJs\": true,

    // Bundler mode
    \"moduleResolution\": \"bundler\",
    \"allowImportingTsExtensions\": true,
    \"verbatimModuleSyntax\": true,
    \"noEmit\": true,

    // Best practices
    \"strict\": true,
    \"skipLibCheck\": true,
    \"noFallthroughCasesInSwitch\": true,

    // Some stricter flags
    \"noUnusedLocals\": true,
    \"noUnusedParameters\": true,
    \"noPropertyAccessFromIndexSignature\": true
}
              }";
            file.write_all(content.as_bytes())?;
        },
        _ => {
            panic!("Invalid template")
        }   
    }

    Ok(())
}

fn init_config() -> Result<Option<Config>, Box<dyn Error>> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    println!("\n\n{} Docs Template\n", style("TSConfig").blue());

    //TODO: maybe implements a dinamic template version
    let template = Select::with_theme(&theme)
        .with_prompt("Choose a template")
        .default(0)
        .item("Recommended")
        .item("Node 21")
        .item("Node 20")
        .item("Node 19")
        .item("Bun")
        .interact()?;

    generate_tsconfig(template)?;

    Ok(Some(Config {
        template
    }))
}

fn main() {
    match init_config() {
        Ok(None) => println!("Aborted."),
        Ok(Some(_)) => println!("Generated {} file in root path", style("tsconfig.json").blue()),
        Err(err) => println!("error: {}", err),
    }
}