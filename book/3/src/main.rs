mod variables;
mod data_types;
mod functions;
mod control_flow;

use variables::main_vars;
use data_types::main_datatypes;
use functions::main_funcs;
use control_flow::main_control_flow;

fn main() {
  println!("1) Variables #########");
  main_vars();
  println!("######################\n");

  println!("2) Data Types ########");
  main_datatypes();
  println!("######################\n");

  println!("3) Functions #########");
  main_funcs();
  println!("######################\n");

  println!("4) Comments ##########");
  println!("######################\n");

  println!("5) Control flow ######");
  main_control_flow();
  println!("######################\n");
}
