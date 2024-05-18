use std::fs::File;
use std::io::Write;
use std::io::Result;

pub const TEMPLATE_NAMES: [&str; 5] = [
  "Recommended",
  "Node 21",
  "Node 20",
  "Node 19",
  "Bun",
];

const TEMPLATES: [&str; 5] = [
  r#"{ 
    "compilerOptions": {
      "target": "es2016",
      "module": "commonjs",
      "esModuleInterop": true,
      "forceConsistentCasingInFileNames": true,
      "strict": true,
      "skipLibCheck": true
    },
    "$schema": "https://json.schemastore.org/tsconfig"
  }"#,
  r#"{
    "$schema": "https://json.schemastore.org/tsconfig",
    "_version": "21.0.0",
              
    "compilerOptions": {
      "lib": ["es2023"],
      "module": "node16",
      "target": "es2022",
                
      "strict": true,
      "esModuleInterop": true,
      "skipLibCheck": true,
      "moduleResolution": "node16"
    }
  }"#,
  r#"{
    "$schema": "https://json.schemastore.org/tsconfig",
    "_version": "20.1.0",
                
    "compilerOptions": {
      "lib": ["es2023"],
      "module": "node16",
      "target": "es2022",
              
      "strict": true,
      "esModuleInterop": true,
      "skipLibCheck": true,
      "moduleResolution": "node16"
    }
  }"#,
  r#"{
    "$schema": "https://json.schemastore.org/tsconfig",
    "_version": "19.1.0",
                
    "compilerOptions": {
      "lib": ["es2023"],
      "module": "node16",
      "target": "es2022",
                
      "strict": true,
      "esModuleInterop": true,
      "skipLibCheck": true,
      "moduleResolution": "node16"
    }
  }"#,
  r#"{
    "$schema": "https://json.schemastore.org/tsconfig",
    "docs": "https://bun.sh/docs/typescript",

    "compilerOptions": {
      "lib": ["ESNext"],
      "target": "ESNext",
      "module": "ESNext",
      "moduleDetection": "force",
      "jsx": "react-jsx",
      "allowJs": true,

      "moduleResolution": "bundler",
      "allowImportingTsExtensions": true,
      "verbatimModuleSyntax": true,
      "noEmit": true,

      "strict": true,
      "skipLibCheck": true,
      "noFallthroughCasesInSwitch": true,

      "noUnusedLocals": true,
      "noUnusedParameters": true,
      "noPropertyAccessFromIndexSignature": true
    }
  }"#,
];

pub fn generate_tsconfig(template: usize) -> Result<()> {
  let mut file = File::create("tsconfig.json")?;
  file.write_all(TEMPLATES[template].as_bytes())?;
  Ok(())
}
