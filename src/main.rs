/*
    Input
    (\a.\b. a b (\t. \f. f)) (\t. \f. t) (\t. \f. t)
*/

/*
    1) Первый app() создаётся на основе всего выражения;
    2) Если в лямбда-функции присутствует более одного аргумента, то она оборачивается в app()
    3) (\a. \b. a b (\t. \f. f)):
        3.1) вызывается лямбда-функция с 4-мя аргументами: \b var(a) var(b) (\t. \f. var(f));
    4) (\t. \f. t) => app( lam( t, lam( f, var(t) ) ) )
*/

/*
    Output
    manysteps(
        app(
            app(
                lam(a, lam( b, app( app( var(a), var(b) ), lam( t, lam(f, var(f)) ) ) ) ),
                lam( t, lam( f, var(t) ) )
            )
            app(
            lam( t, lam( f, var(t) ) )
        ),
        X
    )
*/

// Вся суть в том, что необходимо рассматривать область внутри скобок рекурсивно
// Начинать нужно со скобок наибольшей вложенности, переходя к наименьшей
// (...3 (...1), (...2)) тут цифры - порядок выполнения

fn main() {
    // Get input (whole pseudo code) and create vector of string from it.
    // TODO: read from file!
    // let src = String::from("( /a. /b. a b ( /t. /f. f ) ) ( /t. /f. t )");
    // let src1 = String::from("( \\a. a b )");
    let src1 = String::from(" ( \\a. a b ( \\t. f ) ) ");

    let input = src1.split_whitespace().collect::<Vec<&str>>();
    // Переменная input выглядит вот так
    //["(", "/a.", "/b.", "a", "b", "(", "/t.", "/f.", "f", ")", ")", "(", "/t.", "/f.", "t", ")"]

    // Result string containing intepreted list programm
    let result = translate(input);

    println!("{:?}", result);
}
// Анализируется весь список аргументов
fn translate(source: Vec<&str>) -> String {
    let mut temp_str = String::new();

    // FIXME: исправить повторяющиеся вложения
    for (pos, elem) in source.iter().enumerate() {
        // Проверяем на вхождение в скобки
        if *elem == "(" {
            temp_str = bracket_recursion(&source[pos + 1..&source.len() - 1]);
            // FIXME: временное решение
            break;
        }
    }
    let result = format!("{}", temp_str);
    result
}

// Задумывается, что функция работает только в одной вложенности скобок
fn bracket_recursion(source: &[&str]) -> String {
    let mut is_lambda = false;

    let mut temp_str = String::new();
    // Строка, полученная из рекурсии
    // Вставляется в результат после завершения
    // первой рекурсии
    let mut rec_str_vec: Vec<&str> = Vec::new();

    // Если есть ещё одно вложение скобок, то переходим к нему
    let start = match source.iter().position(|e| e == &"(") {
        Some(p) => p,
        None => source.len(),
    };

    let end = match source.iter().position(|e| e == &")") {
        Some(p) => p,
        None => source.len(),
    };

    let mut _temp = String::new();
    if start != end {
        _temp = bracket_recursion(&source[start + 1..end]);
        rec_str_vec.push(&_temp);
    }

    // Подсчёт аргументов вложенности, считая, что
    // доп вложенность является одним аргументом
    let mut arg_count = 0;
    for elem in source {
        if elem.contains("(") {
            arg_count += 1;
            break;
        } else if elem.contains("\\") {
            continue;
        }
        arg_count += 1;
    }

    let mut arg_str = String::new();
    // FIXME: исправить определение области вложенности
    for (pos, elem) in source[..start].iter().enumerate() {
        // Если элемент является лямбда-функцией
        // то есть начинается со слеша
        if elem.contains("\\") {
            let t = elem.chars().enumerate().nth(1).unwrap().1;
            temp_str.push_str(&format!("{}, ", t));
            is_lambda = true;
        } else {
            if pos == start - 1 {
                arg_str.push_str(&format!("var({})", elem));
            } else {
                arg_str.push_str(&format!("var({}), ", elem));
            }
        }
    }

    let mut result = String::new();
    if is_lambda {
        let mut rec_str = String::new();
        for e in rec_str_vec {
            rec_str.push_str(&format!("{}", e));
        }

        if arg_count >= 2 {
            temp_str.push_str(&format!("app({})", arg_str));
        } else {
            temp_str.push_str(&format!("{}", arg_str));
        }

        if !rec_str.is_empty() {
            result = format!("lam({}, {})", temp_str, rec_str);
        } else {
            result = format!("lam({})", temp_str);
        }
    }
    result
}

#[test]
fn test_simple() {
    let test_string = String::from("( \\a. a b )");
    let iter = test_string.split_whitespace().collect::<Vec<&str>>();
    let result = translate(iter);

    assert_eq!("lam(a, app(var(a), var(b)))", result)
}

#[test]
fn test_complex() {
    let test = String::from(" ( \\a. a b ( \\t. f ) ) ");
    let iter = test.split_whitespace().collect::<Vec<&str>>();
    let result = translate(iter);

    assert_eq!("lam(a, app(var(a), var(b)), lam(t, var(f)))", result);
}
