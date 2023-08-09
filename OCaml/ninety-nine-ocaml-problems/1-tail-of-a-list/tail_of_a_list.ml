(** Tail of a List 
  Problem: Write a function last : 'a list -> 'a option that returns the last element of a list.

  Ex:
    # last ["a" ; "b" ; "c" ; "d"];;
    - : string option = Some "d"
    # last [];;
    - : 'a option = None
*)

(* Recursive f(x) to output the length of a list *)
let rec len a_list =
  match a_list with
  | [] -> 0
  | _ :: t -> 1 + len t;;



(* Our list `x` of letters *)
let x = ["a"; "b"; "c"; "d"];;

(* Use the f(x) `len` to get the length of `x` *)
let n_of_x = len x - 1;;

(* Get the last element in list `x` *)
let tail_of_x = List.nth x n_of_x;;
print_endline tail_of_x


