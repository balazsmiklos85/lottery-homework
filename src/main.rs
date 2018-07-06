use std::env;

fn main() {
    let file_name = get_input_file_name();
    println!("READY");
    //#include <stdio.h>
    //int main() {
    //    int a,b,c,d,e;
    //    printf("READY\n");
    //    int i = 5;
    //    while(i == 5) {
    //        i = scanf("%d %d %d %d %d", &a, &b, &c, &d, &e);
    //        if(i == 5) {
    //            printf("%d %d %d %d\n", 0, 1, 2, 3);
    //        }
    //    }
    //    return 0;
    //}
}

fn get_input_file_name() -> String {
    let mut arguments = env::args();
    arguments.next();
    let result = arguments.next();
    match result {
        Some(r) => return r,
        None => {
            eprintln!("Wrong amount of arguments");
            eprintln!("Usage: ./lottery_homework input_file.name");
            std::process::exit(1);
        },
    }
}