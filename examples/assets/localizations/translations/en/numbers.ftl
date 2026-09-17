# $value (String) - Locale-formatted number.
# $plural (String) - CLDR category keyword.
remaining = { $plural ->
    [one] { $value } item remaining
   *[other] { $value } items remaining
}
empty = Nothing remains. Add an item to continue.
