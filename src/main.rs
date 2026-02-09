use anyhow::{Context, Result};
use exmex::{parse, Express, FlatEx};
use std::io::Write;

fn main() -> Result<()> {
    println!("Добро пожаловать!");
    let func = read_function()?;

    let [first_x, step_target, step_size] = read_parameters()?.try_into()?;
    let max_iter = step_target + first_x;

    let mut step_length = step_target.to_string().len() + 1;
    if step_length < 4 {
        step_length = 4;
    }
    let x_length = (max_iter * step_size).to_string().len() + 1;

    println!(
        "Шаг{}| x{}| Результат",
        if step_length > 3 {
            " ".repeat(step_length - 3)
        } else {
            " ".repeat(step_length)
        },
        " ".repeat(x_length - 1)
    );

    for i in first_x..max_iter {
        let x = (i * step_size) as f64;
        let result = func.eval(&[x]).context("Ошибка вычисления функции!")?;
        let i = (i - first_x + 1) as f64;
        println!(
            "{}{}| {}{}| {}",
            i,
            " ".repeat(step_length - i.to_string().len()),
            x,
            " ".repeat(x_length - x.to_string().len()),
            result
        );
    }
    Ok(())
}

fn read_function() -> Result<FlatEx<f64>> {
    println!("Введите функцию f(x)");
    println!("Пример: x + 10 * 4");
    print!("f(x) = ");
    std::io::stdout().flush()?;
    let mut func = String::new();
    std::io::stdin()
        .read_line(&mut func)
        .context("Ошибка чтения функции!")?;
    let func = func.trim();
    let func = parse::<f64>(func).context("Ошибка парсинга функции!")?;
    Ok(func)
}

fn read_parameters() -> Result<(usize, usize, usize)> {
    println!("Введите параметры рекурсии (через пробел)");
    println!("<чему равен первый x> <количество шагов> <размер шага>");
    println!("Пример: 0 10 1");
    print!("Параметры: ");
    std::io::stdout().flush()?;
    let mut params = String::new();
    std::io::stdin()
        .read_line(&mut params)
        .context("Ошибка чтения параметров!")?;

    let params: Vec<usize> = params
        .trim()
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>, _>>()
        .context("Ошибка парсинга параметров!")?;
    Ok((params[0], params[1], params[2]))
}