fn main() {
      let mut var = 1; 
      let mut found = false;
      while !found {
          var = var + 1;
          println!("{}", var);

          if var == 5 {
              println! ("I encoutered a continue statement");
              continue;
            }
          println!("I did not encounter continue statement");

           if var == 6 {
               found = true;
           }
        }
}