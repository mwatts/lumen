-module(init).
-export([start/0]).

start() ->
  test:caught(fun () ->
    hd([])
  end).
