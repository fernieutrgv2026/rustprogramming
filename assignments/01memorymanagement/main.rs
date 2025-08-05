const FREEZE_POINT_FAHREN: f64 = 32.0;

fn f_to_c(f: f64) -> f64 {
    (f - FREEZE_POINT_FAHREN) * (5.0/9.0)
}

fn c_to_f(c: f64) -> f64 {
    c * (9.0/5.0) + FREEZE_POINT_FAHREN
}

fn main () {
    let mut temp_f: f64 = 45.0;
    let temp_c = f_to_c(temp_f);
    println! {
        "Temperature in Celcius: {:.1}", temp_c
    };
    for i in 1..=5 {
        let curr_fahren = temp_f + i as f64;
        let curr_celc = f_to_c(curr_fahren);
        println! {
            "Temperature in Celcius: {:.1}", curr_celc
        };
    }
}