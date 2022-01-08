// Функция, которая принимает исходную строку и выдаёт результат
pub fn translate_lisp(source: Vec<&str>) -> String {
    // Global scopes
    let mut global_scopes: Vec<&[&str]> = Vec::new();

    // Подсчёт количества глобальных скобок
    // И их запись в вектор
    let mut in_global = false;
    let mut scope_begin = 0;
    let mut temp_scope_end = 0;
    for (pos, elem) in source.iter().enumerate() {
        if elem.contains('(') && !in_global {
            in_global = true;
            scope_begin = pos;
        } else if elem.contains('(') && in_global {
            temp_scope_end = source[pos..]
                .iter()
                .position(|ch| ch.contains(')'))
                .unwrap_or(9999)
                + pos;
        } else if elem.contains(')') && pos > temp_scope_end {
            global_scopes.push(&source[scope_begin + 1..pos]);
            in_global = false;
        }
    }
    let _global_scopes_arg = global_scopes.len();

    // Подсчёт аргуметов
    let mut temp_string_vec: Vec<String> = Vec::new();
    for g_scope in global_scopes {
        for (pos, elem) in g_scope.iter().enumerate() {
            if elem.contains('\\') {
                temp_string_vec.push(construct_lambda(&g_scope[pos..]));
                break;
            }
        }
    }

    let mut _result = String::new();
    for scope in temp_string_vec.iter().enumerate() {
        if scope.0 == 0 {
            _result = format!("{}", scope.1);
        } else {
            _result = format!("{}, {}", _result, scope.1);
        }
    }

    _result
}

// ( \a. \b. a b ( \t. \f. f ) ) ->
// -> lam(a, lam(b, app( var(a), var(b) ), lam(t, lam(f, var(f)))  ) )
// При работе ищет наибольшую вложенность
// и собирает лямбды в обратном порядке
fn construct_lambda(source: &[&str]) -> String {
    // Сборник аргументов лямбды
    let mut temp_arg: Vec<&str> = Vec::new();
    // То, что получено от рекурсии
    let mut inner_res: String = String::new();
    let mut arg_count = 0;
    // Первая буква в лямбде
    let mut lam_str = String::new();

    let mut bgn = source[1..]
        .iter()
        .position(|ch| ch.contains('\\'))
        .unwrap_or(9998);
    bgn += 1;

    if bgn != 9999 {
        inner_res = construct_lambda(&source[bgn..]);
    } else {
        bgn = source.len();
    }

    for elem in &source[..bgn] {
        if elem.contains(')') || elem.contains('(') {
            continue;
        }

        if elem.contains('\\') {
            lam_str = format!("{}", elem.chars().nth(1).unwrap());
        } else {
            arg_count += 1;
            temp_arg.push(elem);
        }
    }

    // Формирование новой строки
    let mut _temp = String::new();
    for e in temp_arg.iter().enumerate() {
        if e.0 == temp_arg.len() - 1 {
            _temp += &("var(".to_string() + &e.1.to_string() + ")");
            continue;
        }
        _temp += &("var(".to_string() + &e.1.to_string() + "), ");
    }

    let mut _result = String::new();
    if arg_count >= 2 && !inner_res.is_empty() && !_temp.is_empty() {
        _result = format!("lam({}, app({}), {})", lam_str, _temp, inner_res);
    } else if arg_count >= 2 && inner_res.is_empty() {
        _result = format!("lam({}, app({}))", lam_str, _temp);
    } else if arg_count < 2 && !inner_res.is_empty() && !_temp.is_empty() {
        _result = format!("lam({}, {}, {})", lam_str, _temp, inner_res);
    } else if arg_count < 2 && !inner_res.is_empty() && _temp.is_empty() {
        _result = format!("lam({}, {})", lam_str, inner_res);
    } else {
        _result = format!("lam({}, {})", lam_str, _temp);
    }

    _result
}
