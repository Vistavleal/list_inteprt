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
    let src = String::from("( /a. /b. a b ( /t. /f. f ) ) ( /t. /f. t )");
    let input = src
        .split_whitespace()
        .collect::<Vec<&str>>();

    // Result string containing intepreted list programm
    let result = translate(input, src.clone());

    println!("{:?}", result);
}
fn translate(collected: Vec<&str>, source: String) -> Result<String, std::io::Error> {
    let mut result = String::new();
    let mut global_scope_counter = 0;

    let mut end_of_scope: usize = 0;
    for (pos, elem) in source.chars().enumerate() {
        let mut scope_len: usize = 0;
        if elem == '(' {
            global_scope_counter += 1;
            end_of_scope = source
                .chars()
                .position(|ch| ch == ')')
                .expect("Not enough brackets");
            // Working inside of brackets
            scope_len = end_of_scope - (pos + 1);
            // Try to find new brackets in the current brackets scope
            let new_brackets = match source[pos + 1..end_of_scope]
                .chars()
                .position(|ch| ch == '(')
            {
                Some(val) => {
                    // Try to find end of new scope
                    let new_end = source[val..end_of_scope - 1]
                        .chars()
                        .position(|ch| ch == ')')
                        .expect("Not enough brackets");
                    // if all is ok then start new recursion
                    bracket_recursion(&source[val..new_end], &collected);
                    continue;
                }
                None => continue,
            };
        }
        //
    }

    Ok(result)
}

fn bracket_recursion(source: &str, collected: &Vec<&str>) -> String {
    let mut result = String::new();
    let mut temp_res = String::new();

    // Когда встречается лямбда-функция необходимо собрать её аргументы
    let mut lamda_comp = false;
    for (pos, elem) in source.chars().enumerate() {
        if lamda_comp {}
        // Если в последовательности есть ещё скобочки, то сразу переходим к ним
        if elem == '(' {
            let end = source
                .chars()
                .position(|ch| ch == ')')
                .expect("Not enought brackets");
            let len = end - (pos + 1);
            temp_res = bracket_recursion(&source[pos + 1..end], &collected);
        } else if elem == '\\' {
            lamda_comp = true;
        }
    }
    result
}

#[test]
fn test_translate() {}
