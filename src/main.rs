use clap::Parser;

use rflashcards::read_lines;

/// parse csv file and convert into a markdown file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// csv file path
   #[arg(short, long)]
   source: String,

   /// target output md file path
   #[arg(short = 'o', long = "output", default_value="")]
   output: String,
}

fn main() {
   let args = Args::parse();

   // File hosts must exist in current path before this produces output
   if let Ok(lines) = read_lines(args.source) {
       // Consumes the iterator, returns an (Optional) String
       for line in lines {
           if let Ok(ip) = line {
               println!("{}", ip);
           }
       }
   } else {
       println!("file not exist");
   }
}


fn read_csv_line() {
}

fn write_md() {
}

func load_history() {
}

fn check_unique() {
}
