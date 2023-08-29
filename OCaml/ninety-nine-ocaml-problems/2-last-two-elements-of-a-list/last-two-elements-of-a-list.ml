(** Last Two Elements of a List
  Problem: Find the last but one (last and penultimate) elements of a list.
  Ex:
    # last_two ["a"; "b"; "c"; "d"];;
    - : (string * string) option = Some ("c", "d")
    # last_two ["a"];;
    - : (string * string) option = None
*)

(* Recursive f(x) to get the last 2 elements in a list *)
let rec last_two a_list = 
  match a_list with
  | [] | [_] -> None
  | [x; y] -> Some (x,y)
  | _ :: r -> last_two r;;  
