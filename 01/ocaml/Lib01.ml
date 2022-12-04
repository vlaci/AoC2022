module Lib01 = struct
  let parse input =
    String.trim input
    |> Str.split (Str.regexp "\n\n")
    |> List.map (fun b ->
           String.trim b |> String.split_on_char '\n' |> List.map int_of_string)

  let rec sum = function [] -> 0 | h :: t -> h + sum t
  let find_max calories = List.map sum calories |> List.fold_left max 0

  let find_max_n calories n =
    List.map sum calories
    |> List.sort (fun a b -> b - a)
    |> List.to_seq |> Seq.take n |> Seq.fold_left ( + ) 0
end

let%test_module _ =
  (module struct
    let input =
      "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n"

    let calories =
      [
        [ 1000; 2000; 3000 ];
        [ 4000 ];
        [ 5000; 6000 ];
        [ 7000; 8000; 9000 ];
        [ 10000 ];
      ]

    let%test "parse" =
      List.equal (List.equal Int.equal) (Lib01.parse input) calories

    let%test "find_max" = Lib01.find_max calories = 24000
    let%test "find_max_n" = Lib01.find_max_n calories 3 = 45000
  end)
