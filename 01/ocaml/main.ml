open Libaoc
open Lib01

let () =
  let input = Libaoc.load_input () in
  let calories = Lib01.parse input in
  let max = Lib01.find_max calories in
  Printf.printf "The elf having the most calories has %i caloris\n" max;
  let total = Lib01.find_max_n calories 3 in
  Printf.printf "The top 3 elves have %i calories in total\n" total
