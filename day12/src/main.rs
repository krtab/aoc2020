fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut direction: i64 = 0;
    let mut east: i64 = 0;
    let mut north = 0;
    for l in buf.lines() {
        let (command, arg) = l.split_at(1);
        let arg: i64 = arg.parse()?;
        match command {
            "N" => north += arg,
            "S" => north -= arg,
            "E" => east += arg,
            "W" => east -= arg,
            "L" => direction = (direction + arg) % 360,
            "R" => direction = (direction - arg) % 360,
            "F" => {
                let f = (direction as f64).to_radians();
                north += (f.sin() as i64) * arg;
                east += (f.cos() as i64) * arg;
            }
            _ => unimplemented!(),
        };
    }
    println!("Answer1: {}", north.abs() + east.abs());
    let mut east: i64 = 0;
    let mut north = 0;
    let mut wpeast = 10;
    let mut wpnorth = 1;
    for l in buf.lines() {
        let (command, arg) = l.split_at(1);
        let arg: i64 = arg.parse()?;
        match command {
            "N" => wpnorth += arg,
            "S" => wpnorth -= arg,
            "E" => wpeast += arg,
            "W" => wpeast -= arg,
            "L" | "R" => {
                let arg = if command == "R" { 360 - arg } else { arg };
                let (new_wpeast, new_wpnorth) = match arg {
                    90 => (-wpnorth, wpeast),
                    180 => (-wpeast, -wpnorth),
                    270 => (wpnorth, -wpeast),
                    _ => unimplemented!(),
                };
                wpnorth = new_wpnorth;
                wpeast = new_wpeast;
            }
            "F" => {
                north +=  arg * wpnorth;
                east +=  arg * wpeast;
            }
            _ => unimplemented!(),
        };
    }
    println!("Answer2: {}", north.abs() + east.abs());
    Ok(())
}
