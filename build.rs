extern crate lalrpop;

fn main() {
    // Set the input directory for lalrpop-files to src/grammar
    let mut lalrpop_config = lalrpop::Configuration::new();
    lalrpop_config.set_in_dir("src/grammar/");
    lalrpop_config.process().unwrap()
}
