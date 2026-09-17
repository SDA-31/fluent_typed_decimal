# $value (String) - Locale-formatted number.
# $plural (String) - CLDR category keyword.
remaining = { $plural ->
    [zero] تبقى { $value } من العناصر
    [one] تبقى { $value } من العناصر
    [two] تبقى { $value } من العناصر
    [few] تبقت { $value } عناصر
    [many] تبقى { $value } عنصرًا
   *[other] تبقى { $value } من العناصر
}
empty = لم يتبق شيء. أضف عنصرًا للمتابعة.
