test_stdout!(
    with_big_integer_right_returns_big_integer,
    "9838263505978427528\ntrue\n"
);
test_stdout!(
    with_big_integer_right_returns_small_integer_integer,
    "8\ntrue\n"
);
test_stdout!(
    with_small_integer_right_returns_small_integer_integer,
    "8\ntrue\n"
);
test_stdout!(with_same_right_returns_same_integer, "true\n");
