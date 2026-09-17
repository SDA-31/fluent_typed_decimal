# $value (String) - Locale-formatted number.
# $plural (String) - CLDR category keyword.
remaining = { $plural ->
    [one] Остался { $value } предмет
    [few] Осталось { $value } предмета
    [many] Осталось { $value } предметов
   *[other] Осталось { $value } предмета
}
empty = Ничего не осталось. Добавьте предмет, чтобы продолжить.
