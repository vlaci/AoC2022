module Libaoc = struct
  let load_input () =
    let filename = ref None in
    let specs = [] in
    let usage = "<input file>" in
    let anon n = filename := Some n in
    Arg.parse specs anon usage;
    match !filename with
    | None ->
        Arg.usage specs usage;
        invalid_arg "<input file> is required"
    | Some f -> In_channel.with_open_text f In_channel.input_all
end
