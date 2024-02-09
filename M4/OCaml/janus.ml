type var = string
type env = (var, int) Hashtbl.t
type prog = (env * bool) -> unit
type expr = env -> int


module Mod =
struct
type t =
| Add
| Sub
| Mul
| Div
| Xor

let eval op v1 v2 b =
  match op with
  | Add -> if b then v1+v2 else v1-v2
  | Sub -> if b then v1-v2 else v1+v2
  | Mul -> if b then v1*v2 else (assert (v1 mod v2 = 0); v1/v2)
  | Div -> if b then (assert (v1 mod v2 = 0); v1/v2) else v1*v2
  | Xor -> if v1 <> v2 then 1 else 0 

end

module Exo3 =
struct
let null () =
  fun (_, _) -> ()

let assign x1 m expr =
  fun (env, b) -> 
  let v1 = Hashtbl.find env x1 in
  let newv1 = Mod.eval m v1 (expr env) b in
  Hashtbl.replace env x1 newv1

let swap x1 x2 =
  fun (env, _) ->
    let v1 = Hashtbl.find env x1 in
    let v2 = Hashtbl.find env x2 in
    (Hashtbl.replace env x1 v2; Hashtbl.replace env x2 v1)

let seq p1 p2 =
  fun (env, b) ->
    if b then (p1 (env, b); p2 (env, b)) else (p2 (env, b); p1 (env, b))

let conditional pre p_then p_else post =
  fun (env, b) ->
    if b
      then (if pre env = 1
              then p_then (env, b)
              else p_else (env, b);
            assert (post env = 1))
      else (if post env = 1
              then p_then (env, b)
              else p_else (env, b);
            assert (pre env = 1))

let repeat pre p_forward p_backward post =
  let exception Break in
  fun (env, b) ->
    if b
      then (assert (pre env = 1);
            try
            while true
            do
              p_forward (env, b);
              if post env = 1 then raise Break;
              p_backward (env, b);
              assert (pre env = 0);
            done
          with Break -> ())
      else (assert (post env = 1);
            try
            while true
            do
              p_backward (env, b);
              if pre env = 1 then raise Break;
              p_forward (env, b);
              assert (post env = 0);
            done
          with Break -> ())

let fibonacci x1 x2 n =
     seq (assign x1 Mod.Add (fun _ -> 1))
    (seq (assign x2 Mod.Add (fun _ -> 1))
         (repeat
            (fun env -> if Hashtbl.find env x1 = Hashtbl.find env x2 then 1 else 0)
            (seq (assign x1 Mod.Add (fun env -> Hashtbl.find env x2))
            (seq (swap x1 x2)
                 (assign n Mod.Sub (fun _ -> 1))))
            (null ())
            (fun env -> if Hashtbl.find env n = 0 then 1 else 0)
         )
    )

  let test_fibonacci () =
    let x1 = "x1" in
    let x2 = "x2" in
    let n = "n" in
    let env = Hashtbl.create 23 in
    (
      Hashtbl.add env x1 0;
      Hashtbl.add env x2 0;
      Hashtbl.add env n 4;
      let prog = fibonacci x1 x2 n in
      prog (env, true);
      Format.printf "%s = %d, %s = %d, %s = %d\n"
            x1 (Hashtbl.find env x1)
            x2 (Hashtbl.find env x2)
            n (Hashtbl.find env n)
    )

    let test_reverse_fibonacci () =
      let x1 = "x1" in
      let x2 = "x2" in
      let n = "n" in
      let env = Hashtbl.create 23 in
      (
        Hashtbl.add env x1 5;
        Hashtbl.add env x2 8;
        Hashtbl.add env n 0;
        let prog = fibonacci x1 x2 n in
        prog (env, false);
        Format.printf "%s = %d, %s = %d, %s = %d\n"
              x1 (Hashtbl.find env x1)
              x2 (Hashtbl.find env x2)
              n (Hashtbl.find env n)
      )
    end

let _ =
  (
    Exo3.test_fibonacci ();
    Exo3.test_reverse_fibonacci ()
  )