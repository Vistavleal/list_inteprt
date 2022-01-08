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
mod algorythm;
use algorythm::*;

fn main() {
    // Get input (whole pseudo code) and create vector of string from it.
    let src1 = String::from("( \\a. \\b. a b ( \\t. \\f. f ) ) ( \\t. \\f. t )");
    // let src1 = String::from("( \\a. a b )");
    // let src1 = String::from(" ( \\a. a b ( \\t. f ) ) ");

    let input = src1.split_whitespace().collect::<Vec<&str>>();
    // Переменная input выглядит вот так
    //["(", "/a.", "/b.", "a", "b", "(", "/t.", "/f.", "f", ")", ")", "(", "/t.", "/f.", "t", ")"]

    // Result string containing intepreted list programm
    let result = translate_lisp(input);

    println!("Source: {}", src1);
    println!("Result: {}", result);
}

#[test]
fn test_simple() {
    let test_string = String::from("( \\a. a b )");
    let iter = test_string.split_whitespace().collect::<Vec<&str>>();
    let result = translate_lisp(iter);

    assert_eq!("lam(a, app(var(a), var(b)))", result)
}

#[test]
fn test_complex() {
    let test = String::from(" ( \\a. a b ( \\t. f ) ) ");
    let iter = test.split_whitespace().collect::<Vec<&str>>();
    let result = translate_lisp(iter);

    assert_eq!("lam(a, app(var(a), var(b)), lam(t, var(f)))", result);
}

#[test]
fn test_multiple_lambda() {
    let test = String::from(" ( \\a. \\b. \\f. \\t. d c ) ");
    let iter = test.split_whitespace().collect::<Vec<&str>>();
    let result = translate_lisp(iter);

    assert_eq!("lam(a, lam(b, lam(f, lam(t, app(var(d), var(c))))))", result);
}

#[test]
fn test_complex_multiple_lambda() {
    let test = String::from(" ( \\a. \\b. \\f. \\t. d c ) ( \\a. \\b. \\f. \\t. d c ) ");
    let iter = test.split_whitespace().collect::<Vec<&str>>();
    let result = translate_lisp(iter);

    assert_eq!("lam(a, lam(b, lam(f, lam(t, app(var(d), var(c)))))), lam(a, lam(b, lam(f, lam(t, app(var(d), var(c))))))", result);
}