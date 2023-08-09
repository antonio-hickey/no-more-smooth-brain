(** Tail of a List 
  Problem: Write a function last : 'a list -> 'a option that returns the last element of a list.

  Ex:
    # last ["a" ; "b" ; "c" ; "d"];;
    - : string option = Some "d"
    # last [];;
    - : 'a option = None
*)

(* Recursive f(x) to output the length of a list *)
let rec last a_list =
  match a_list with
  | [] -> None
  | [ x ] -> Some x
  | _ :: remaining -> last remaining;;
 
