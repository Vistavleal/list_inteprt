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

    let global_scopes_arg = global_scopes.len();
    let mut scopes_arg = 0;

    // Подсчёт аргуметов
    // TODO
    for g_scope in global_scopes {
        let mut skip_scopes = 9999;
        for (pos, elem) in g_scope.iter().enumerate() {
            if pos <= skip_scopes && skip_scopes != 9999 {
                if elem.contains('\\') {
                    skip_scopes = g_scope[pos + 1..]
                        .iter()
                        .position(|ch| ch.contains(')'))
                        .unwrap_or(9999);
                    scopes_arg += 1;
                } else {
                    scopes_arg += 1;
                }
            }
        }
    }

    String::new()
}

// Отвечает за формирование выражений
fn rec(source: &[&str]) -> String {
    let mut result = String::new();
    let mut arg_str: Vec<String> = Vec::new();

    for (pos, elem) in source.iter().enumerate() {
        if elem.contains('(') {
            let scope_end = source[pos + 1..]
                .iter()
                .position(|ch| ch.contains(')'))
                .unwrap();
            arg_str.push(rec(&source[pos+1..scope_end]));
        }
    }

    // Формирование результата

    String::new()
}

// TODO:
fn construct_lambda(source: &[&str]) -> String {
    

    String::new()
}
