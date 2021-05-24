open Owl;;
open Owl_plplot;;

let plot_fun () =
  let f x = Maths.sin x /. x in
  let h = Plot.create "plot_001.png" in
  Plot.set_title h "Function: f(x) = sine x / x";
  Plot.set_xlabel h "x-axis";
  Plot.set_ylabel h "y-axis";
  Plot.set_font_size h 8.;
  Plot.set_pen_size h 3.;
  Plot.plot_fun ~h f 1. 15.;

Plot.output h
;;

let () = plot_fun ()