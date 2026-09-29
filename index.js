import { create_stuff, run_alert } from "./domcrate";

const rust = import("./domcrate");

rust.then(() => {
  create_stuff();
  run_alert("Alert from JS");
});
