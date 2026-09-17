# $value (String) - Locale-formatted number.
# $plural (String) - CLDR category keyword.
remaining = { $plural ->
    [one] Queda { $value } elemento
   *[other] Quedan { $value } elementos
}
empty = No queda nada. Añade un elemento para continuar.
