use std::fs;

fn main() -> Result<(), std::io::Error> {
  let input: Vec<u8> = fs::read("./assets/in.bin")?;

  let size_before: usize = input.len();
  let size_expected: usize = &size_before - &size_before / 4;

  let print_width = 5;
  println!("Size before:\t{:>print_width$} bytes", &size_before);
  println!("Size expected:\t{:>print_width$} bytes", &size_expected);

  let mut output: Vec<u8> = Vec::with_capacity(size_expected);

  for (index, byte) in input.iter().enumerate() {
    if (index + 1) % 4 != 0 {
      output.push(*byte);
    }
  }

  fs::write("./assets/out.bin", output)?;

  println!(
    "Size After:\t{:>print_width$} bytes",
    fs::metadata("./assets/out.bin")
      .expect("Could not read file metadata")
      .len()
  );

  Ok(())
}
